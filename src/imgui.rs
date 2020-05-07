// wengwengweng

use crate::Result;
use super::*;
use gl::VertexLayout;

use std::time::Instant;

pub use imgui_lib::Window;
pub use imgui_lib::Ui;
pub use imgui_lib::Condition;
pub use imgui_lib::im_str;

use glutin::window::Window as WinitWindow;
use imgui_lib::DrawCmd;
use imgui_lib::DrawCmdParams;
use imgui_lib::Context;

impl gfx::Vertex {
	fn from_imgui_vert(v: imgui_lib::DrawVert) -> Self {
		return Self {
			pos: vec3!(v.pos[0], v.pos[1], 0.0),
			uv: vec2!(v.uv[0], v.uv[1]),
			normal: vec3!(0, 0, -1),
			color: rgba!(
				v.col[0] as f32 / 255.0,
				v.col[1] as f32 / 255.0,
				v.col[2] as f32 / 255.0,
				v.col[3] as f32 / 255.0,
			),
		};
	}
}

pub(crate) struct Imgui {
	ctx: Context,
	tex: gfx::Texture,
	last_frame: Instant,
	pipeline: gl::Pipeline<gfx::Vertex, gfx::Uniform>,
	renderer: gl::BatchedMesh<gfx::Vertex, gfx::Uniform>,
}

impl Imgui {

	pub fn new(ctx: &Ctx) -> Result<Self> {

		use imgui_lib::BackendFlags;

		let mut imgui = Context::create();
		let io = imgui.io_mut();
		let dpi = ctx.dpi();
		let (w, h) = (ctx.width(), ctx.height());

		io.display_framebuffer_scale = [dpi as f32, dpi as f32];
		io.display_size = [w as f32 * dpi, h as f32 * dpi];
		io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
		io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);

		imgui.set_ini_filename(None);

		let tex = {

			let mut atlas = imgui.fonts();
			let tex_data = atlas.build_rgba32_texture();
			let width = tex_data.width as i32;
			let height = tex_data.height as i32;

			gfx::Texture::from_pixels(ctx, width, height, tex_data.data)

		}?;

		use res::shader::*;

		let vert_src = TEMPLATE_VERT.replace("###REPLACE###", DEFAULT_VERT);
		let frag_src = TEMPLATE_FRAG.replace("###REPLACE###", DEFAULT_FRAG);

		let pipeline = gl::Pipeline::new(&ctx.gl, &vert_src, &frag_src)?;

		return Ok(Self {
			ctx: imgui,
			last_frame: Instant::now(),
			tex: tex,
			pipeline: pipeline,
			renderer: gl::BatchedMesh::<gfx::Vertex, gfx::Uniform>::new(&ctx.gl, gfx::DRAW_COUNT, gfx::DRAW_COUNT)?,
		});

	}

	pub fn event(&mut self, ctx: &app::Ctx, e: &input::Event) {

		use input::*;
		use input::Event::*;

		let io = self.ctx.io_mut();

		match *e {
			Resize(w, h) => {
				io.display_size = [w as f32 * ctx.dpi(), h as f32 * ctx.dpi()];
			},
			CharInput(ch) => {
				io.add_input_character(ch);
            },
			KeyPress(k) => {
				match k {
					Key::LShift | Key::RShift => io.key_shift = true,
					Key::LCtrl | Key::RCtrl => io.key_ctrl = true,
					Key::LAlt | Key::RAlt => io.key_alt = true,
					Key::LMeta | Key::RMeta => io.key_super = true,
					_ => {},
				}
			},
			KeyRelease(k) => {
				match k {
					Key::LShift | Key::RShift => io.key_shift = false,
					Key::LCtrl | Key::RCtrl => io.key_ctrl = false,
					Key::LAlt | Key::RAlt => io.key_alt = false,
					Key::LMeta | Key::RMeta => io.key_super = false,
					_ => {},
				}
			},
			MouseMove(_) => {
				let mpos = ctx.mouse_pos();
				let (w, h) = (ctx.width() as f32, ctx.height() as f32);
				let mpos = vec2!(w / 2.0 + mpos.x, h / 2.0 - mpos.y);
				io.mouse_pos = [mpos.x, mpos.y];
			},
			MousePress(m) => {
				match m {
					Mouse::Left => io.mouse_down[0] = true,
					Mouse::Right => io.mouse_down[1] = true,
					Mouse::Middle => io.mouse_down[2] = true,
				}
			},
			MouseRelease(m) => {
				match m {
					Mouse::Left => io.mouse_down[0] = false,
					Mouse::Right => io.mouse_down[1] = false,
					Mouse::Middle => io.mouse_down[2] = false,
				}
			},
			Scroll(delta, _) => {
				io.mouse_wheel_h += delta.x;
				io.mouse_wheel += delta.y;
			},
			_ => {},
		}

	}

	pub fn render(
		&mut self,
		f: impl FnOnce(&mut imgui_lib::Ui) -> Result<()>,
	) -> Result<()> {

		self.last_frame = self.ctx
			.io_mut()
			.update_delta_time(self.last_frame);

		let mut ui = self.ctx.frame();

		f(&mut ui)?;

		let [w, h] = ui.io().display_size;
		let [sw, sh] = ui.io().display_framebuffer_scale;
		let (fw, fh) = (w / sw, h / sh);

		let proj = mat4![
			2.0 / fw, 0.0, 0.0, 0.0,
			0.0, 2.0 / -fh, 0.0, 0.0,
			0.0, 0.0, -1.0, 0.0,
			-1.0, 1.0, 0.0, 1.0,
		];

		let draw_data = ui.render();

		for draw_list in draw_data.draw_lists() {

			let vbuf = draw_list.vtx_buffer();
			let ibuf = draw_list
				.idx_buffer()
				.into_iter()
				.map(|i| *i as u32)
				.collect::<Vec<u32>>();

			let mut vqueue = Vec::with_capacity(vbuf.len());

			for v in vbuf {
				vqueue.push(gfx::Vertex::from_imgui_vert(*v));
			}

			self.renderer.push(
				gl::Primitive::Triangle,
				&vqueue,
				&ibuf,
				&self.pipeline,
				&gfx::Uniform {
					model: mat4!(),
					proj: proj,
					view: mat4!(),
					color: rgba!(1),
					tex: self.tex.clone(),
					custom: None,
				},
			)?;

		}

		self.renderer.flush();

		return Ok(());

	}

}

