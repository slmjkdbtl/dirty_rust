// wengwengweng

use super::*;
use crate::Result;

use imgui::DrawVert;

// impl gl::Vertex2D for DrawVert {
	// ...
// }

pub struct Renderer {
	vbuf: gl::VertexBuffer<gfx::Vertex2D>,
	ibuf: gl::IndexBuffer,
	font_tex: gl::Texture,
}

impl Renderer {

	pub fn init(gl: &gl::Device, ctx: &mut imgui::Context) -> Result<Self> {

		let vbuf = gl::VertexBuffer::new(gl, 999, gl::BufferUsage::Dynamic)?;
		let ibuf = gl::IndexBuffer::new(gl, 999, gl::BufferUsage::Dynamic)?;

		let mut atlas = ctx.fonts();
		let texture = atlas.build_rgba32_texture();
		let font_tex = gl::Texture::init(&gl, texture.width as i32, texture.height as i32, texture.data)?;

		atlas.tex_id = (font_tex.id as usize).into();

		return Ok(Self {
			vbuf: vbuf,
			ibuf: ibuf,
			font_tex: font_tex,
		});

	}

	pub fn render<'ui>(&self, gl: &gl::Device, ui: imgui::Ui<'ui>) -> Result<()> {

		let draw_data = ui.render();

		for draw_list in draw_data.draw_lists() {

			let vtx_buffer = draw_list.vtx_buffer();
			let idx_buffer = draw_list.idx_buffer();

// 			self.vbuf.data_raw(vtx_buffer);
// 			self.ibuf.data_raw(idx_buffer);

		}

		return Ok(());

	}

}

