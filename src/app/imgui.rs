// wengwengweng

use crate::Result;
use super::*;
use gl::VertexLayout;

use std::time::Instant;

use glutin::window::Window;
use imgui_lib::im_str;
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

pub struct Imgui {
	ctx: Context,
	platform: WinitPlatform,
	tex: gfx::Texture,
	last_frame: Instant,
}

impl Imgui {

	pub fn new(gl: &gl::Device, window: &Window) -> Result<Self> {

		let mut ctx = Context::create();
		let mut platform = WinitPlatform::init(&mut ctx);

		ctx.set_ini_filename(None);
		platform.attach_window(ctx.io_mut(), &window, imgui_winit::HiDpiMode::Locked(1.0),);

		let tex = {

			let mut atlas = ctx.fonts();
			let tex_data = atlas.build_rgba32_texture();
			let width = tex_data.width as i32;
			let height = tex_data.height as i32;
			let tex = gl::Texture2D::from(&gl, width, height, tex_data.data).unwrap();

			gfx::Texture::from_gl_tex(tex)

		};

		return Ok(Self {
			ctx: ctx,
			platform: platform,
			last_frame: Instant::now(),
			tex: tex,
		});

	}

	pub fn handle_event(&mut self, ctx: &Ctx, e: &glutin::event::Event<()>) {
		self.platform.handle_event(self.ctx.io_mut(), ctx.windowed_ctx.window(), &e);
	}

	pub fn render(
		&mut self,
		ctx: &mut Ctx,
		f: impl FnOnce(&mut imgui_lib::Ui) -> (),
	) -> Result<()> {

		let window = ctx.windowed_ctx.window();

		self.platform
			.prepare_frame(self.ctx.io_mut(), &window)
            .map_err(|_| format!("failed to prepare imgui frame"))?;

		self.last_frame = self.ctx
			.io_mut()
			.update_delta_time(self.last_frame);

		let mut ui = self.ctx.frame();

		f(&mut ui);

		imgui_lib::Window::new(im_str!("window"))
			.size([300.0, 100.0], imgui_lib::Condition::FirstUseEver)
			.build(&ui, || {
				ui.text(im_str!("Hello world!"));
			});

		self.platform.prepare_render(&ui, &window);

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

			let (w, h) = (ctx.width() as f32, ctx.height() as f32);

			let proj = mat4![
				2.0 / w, 0.0, 0.0, 0.0,
				0.0, 2.0 / -h, 0.0, 0.0,
				0.0, 0.0, -1.0, 0.0,
				-1.0, 1.0, 0.0, 1.0,
			];

			ctx.flush();

			ctx.renderer_2d.push(
				gl::Primitive::Triangle,
				&vqueue,
				&ibuf,
				&ctx.cur_pipeline_2d,
				&gfx::Uniform2D {
					proj: proj,
					tex: self.tex.clone(),
					custom: None,
				},
			)?;

			ctx.flush();

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

		return Ok(());

	}

}

