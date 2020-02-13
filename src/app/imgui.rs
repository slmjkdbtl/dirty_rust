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
use imgui_winit::WinitPlatform;

impl gfx::Vertex2D {
	fn from_imgui_vert(v: imgui_lib::DrawVert) -> Self {
		return Self {
			pos: vec3!(v.pos[0], v.pos[1], 0.0),
			uv: vec2!(v.uv[0], v.uv[1]),
			color: rgba!(
				v.col[0] as f32 / 255.0,
				v.col[1] as f32 / 255.0,
				v.col[2] as f32 / 255.0,
				v.col[3] as f32 / 255.0,
			),
		};
	}
}

pub(super) struct Imgui {
	ctx: Context,
	platform: WinitPlatform,
	tex: gfx::Texture,
	last_frame: Instant,
	pipeline: gl::Pipeline<gfx::Vertex2D, gfx::Uniform2D>,
	renderer: gl::BatchedMesh<gfx::Vertex2D, gfx::Uniform2D>,
}

impl Imgui {

	pub fn new(gl: &gl::Device, window: &WinitWindow) -> Result<Self> {

		let mut ctx = Context::create();
		let mut platform = WinitPlatform::init(&mut ctx);

		ctx.set_ini_filename(None);
// 		platform.attach_window(ctx.io_mut(), &window, imgui_winit::HiDpiMode::Default,);
		platform.attach_window(ctx.io_mut(), &window, imgui_winit::HiDpiMode::Locked(1.0));

		let tex = {

			let mut atlas = ctx.fonts();
			let tex_data = atlas.build_rgba32_texture();
			let width = tex_data.width as i32;
			let height = tex_data.height as i32;
			// TODO: no unwrap
			let tex = gl::Texture2D::from(&gl, width, height, tex_data.data).unwrap();

			gfx::Texture::from_gl_tex(tex)

		};

		use res::shader::*;

		let vert_2d_src = TEMPLATE_2D_VERT.replace("###REPLACE###", DEFAULT_2D_VERT);
		let frag_2d_src = TEMPLATE_2D_FRAG.replace("###REPLACE###", DEFAULT_2D_FRAG);

		let pipeline = gl::Pipeline::new(&gl, &vert_2d_src, &frag_2d_src)?;

		return Ok(Self {
			ctx: ctx,
			platform: platform,
			last_frame: Instant::now(),
			tex: tex,
			pipeline: pipeline,
			renderer: gl::BatchedMesh::<gfx::Vertex2D, gfx::Uniform2D>::new(&gl, DRAW_COUNT, DRAW_COUNT)?,
		});

	}

	pub fn handle_event(&mut self, window: &WinitWindow, e: &glutin::event::Event<()>) {
		self.platform.handle_event(self.ctx.io_mut(), window, &e);
	}

	pub fn render(
		&mut self,
		window: &WinitWindow,
		f: impl FnOnce(&mut imgui_lib::Ui) -> Result<()>,
	) -> Result<()> {

		self.platform
			.prepare_frame(self.ctx.io_mut(), &window)
            .map_err(|_| format!("failed to prepare imgui frame"))?;

		self.last_frame = self.ctx
			.io_mut()
			.update_delta_time(self.last_frame);

		let mut ui = self.ctx.frame();

		f(&mut ui)?;

		self.platform.prepare_render(&ui, &window);

// 		let [w, h] = ui.io().display_size;
// 		let [sw, sh] = ui.io().display_framebuffer_scale;
// 		let (fw, fh) = (w * sw, h * sh);

		let dpi = window.scale_factor();
		let size = window.inner_size().to_logical::<f32>(dpi);

		let proj = mat4![
			2.0 / size.width, 0.0, 0.0, 0.0,
			0.0, 2.0 / -size.height, 0.0, 0.0,
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

			let mut vqueue = Vec::with_capacity(vbuf.len() * gfx::Vertex2D::STRIDE);

			for v in vbuf {
				gfx::Vertex2D::from_imgui_vert(*v).push(&mut vqueue);
			}

			self.renderer.push(
				gl::Primitive::Triangle,
				&vqueue,
				&ibuf,
				&self.pipeline,
				&gfx::Uniform2D {
					proj: proj,
					tex: self.tex.clone(),
					custom: None,
				},
			)?;

			for cmd in draw_list.commands() {
				match cmd {
					DrawCmd::Elements {
						count,
						cmd_params: DrawCmdParams {
							clip_rect: [x, y, z, w],
							texture_id,
							idx_offset,
							..
						}
					} => {
					},
					_ => {},
				}
			}

		}

		self.renderer.flush();

		return Ok(());

	}

}

