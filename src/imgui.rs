// wengwengweng

use instant::Instant;
pub use libimgui::Window;
pub use libimgui::Ui;
pub use libimgui::Condition;
pub use libimgui::im_str;

use crate::*;
use math::*;
use input::*;
use gfx::*;

// TODO: clean up width / height / DPI
// TODO: DrawCmd

impl Vertex {
	fn from_imgui(v: &libimgui::DrawVert) -> Self {
		return Self {
			pos: vec3!(v.pos[0], v.pos[1], 0.0),
			uv: vec2!(v.uv[0], v.uv[1]),
			normal: vec3!(0, 0, 1),
			color: Color::from_u8(v.col[0], v.col[1], v.col[2], v.col[3]),
		};
	}
}

pub struct Imgui {
	ctx: libimgui::Context,
	tex: Texture,
// 	last_frame: Instant,
}

impl Imgui {

	pub fn new(d: &Ctx) -> Result<Self> {

		use libimgui::BackendFlags;

		let mut ctx = libimgui::Context::create();
		let io = ctx.io_mut();
		let dpi = d.gfx.dpi();
		let (w, h) = (d.gfx.width(), d.gfx.height());

		io.display_framebuffer_scale = [dpi as f32, dpi as f32];
		io.display_size = [w as f32 * dpi, h as f32 * dpi];
		io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
		io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);

		ctx.set_ini_filename(None);

		let tex = {

			let mut atlas = ctx.fonts();
			let tex_data = atlas.build_rgba32_texture();
			let width = tex_data.width as i32;
			let height = tex_data.height as i32;

			Texture::from_raw(d.gfx, width, height, tex_data.data)

		}?;

		return Ok(Self {
			ctx: ctx,
			tex: tex,
// 			last_frame: Instant::now(),
		});

	}

	pub fn event(&mut self, d: &mut Ctx, e: &Event) {

		use Event::*;

		let io = self.ctx.io_mut();

		match *e {
			Resize(w, h) => {
				io.display_size = [w as f32 * d.gfx.dpi(), h as f32 * d.gfx.dpi()];
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
				let mpos = d.window.mouse_pos();
				let (w, h) = (d.gfx.width() as f32, d.gfx.height() as f32);
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
			Wheel(delta, _) => {
				io.mouse_wheel_h += delta.x;
				io.mouse_wheel += delta.y;
			},
			_ => {},
		}

	}

	pub fn render(
		&mut self,
		d: &mut Ctx,
		f: impl FnOnce(&mut libimgui::Ui) -> Result<()>,
	) -> Result<()> {

// 		self.last_frame = self.ctx
// 			.io_mut()
// 			.update_delta_time(self.last_frame);

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

		let cam = StaticCam {
			proj: proj,
			view: mat4!(),
		};

		let draw_data = ui.render();

		let tex = self.tex.clone();

		d.gfx.use_cam(&cam, |gfx| {

			for draw_list in draw_data.draw_lists() {

				let vertices = draw_list
					.vtx_buffer()
					.into_iter()
					.map(Vertex::from_imgui)
					.collect::<Vec<Vertex>>();

				let indices = draw_list
					.idx_buffer()
					.into_iter()
					.map(|i| *i as u32)
					.collect::<Vec<u32>>();

				gfx.draw(&shapes::raw(&vertices, &indices).texture(&tex))?;

				for cmd in draw_list.commands() {
					match cmd {
						libimgui::DrawCmd::Elements { count, .. } => {},
						_ => {},
					}
				}

			}

			return Ok(());

		})?;

		return Ok(());

	}

}



