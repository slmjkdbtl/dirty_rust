// wengwengweng

use std::mem;
use std::rc::Rc;

use glow::Context;
type GLCtx = glow::native::Context;
type BufferID = <GLCtx as Context>::Buffer;
type ProgramID = <GLCtx as Context>::Program;
type TextureID = <GLCtx as Context>::Texture;
type FramebufferID = <GLCtx as Context>::Framebuffer;
type VertexArrayID = <GLCtx as Context>::VertexArray;

use crate::Result;
use crate::math::*;

pub struct Device {
	gl: Rc<GLCtx>,
}

impl Device {

	pub fn from_loader<F: FnMut(&str) -> *const std::os::raw::c_void>(f: F) -> Self {
		return Self {
			gl: Rc::new(GLCtx::from_loader_function(f)),
		};
	}

	pub fn new_vbuf(&mut self, count: usize, stride: usize, usage: BufferUsage) -> Result<VertexBuffer> {

		unsafe {

			let id = self.gl.create_buffer()?;

			let handle = VertexBuffer {
				id: id,
				count: count,
				stride: stride,
			};

			self.bind_vbuf(Some(&handle));

			self.gl.buffer_data_size(
				glow::ARRAY_BUFFER,
				(count * mem::size_of::<f32>()) as i32,
				usage.into(),
			);

			return Ok(handle);

		}

	}

	pub fn set_vbuf_data(&mut self, buf: &VertexBuffer, data: &[f32], offset: usize) {

		unsafe {

			self.bind_vbuf(Some(buf));

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.gl.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(offset * mem::size_of::<f32>()) as i32,
				byte_slice,
			);

		}

	}

	fn bind_vbuf(&mut self, buf: Option<&VertexBuffer>) {
		unsafe {
			self.gl.bind_buffer(glow::ARRAY_BUFFER, buf.map(|b| b.id));
		}
	}

	fn bind_ibuf(&mut self, buf: Option<&IndexBuffer>) {
		unsafe {
			self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, buf.map(|b| b.id));
		}
	}

	fn bind_shader(&mut self, shader: Option<&Shader>) {
		unsafe {
			self.gl.use_program(shader.map(|s| s.id));
		}
	}

	fn bind_tex(&mut self, tex: Option<&Texture>) {
		unsafe {
			self.gl.active_texture(glow::TEXTURE0);
			self.gl.bind_texture(glow::TEXTURE_2D, tex.map(|t| t.id));
		}
	}

	pub fn bind_framebuffer(&mut self, fbuf: Option<&Framebuffer>) {
		unsafe {
			self.gl.bind_framebuffer(glow::FRAMEBUFFER, fbuf.map(|f| f.id));
		}
	}

	pub fn draw_elements(
		&mut self,
		vbuf: &VertexBuffer,
		ibuf: &IndexBuffer,
		tex: &Texture,
		shader: &Shader,
		count: i32,
	) {

		unsafe {

			self.bind_vbuf(Some(vbuf));
//			self.bind_ibuf(Some(ibuf));
//			self.bind_tex(Some(tex));
//			self.bind_shader(Some(shader));

			self.gl.draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, 0);

		}
	}

	pub fn clear_color(&self, c: Color) {
		unsafe {
			self.gl.clear_color(c.r, c.g, c.b, c.a);
		}
	}

	pub fn clear(&self) {
		unsafe {
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}

}

pub struct VertexBuffer {
	id: BufferID,
	count: usize,
	stride: usize,
}

pub struct IndexBuffer {
	id: BufferID,
}

pub struct Texture {
	id: TextureID,
}

pub struct Shader {
	id: ProgramID,
}

pub struct Framebuffer {
	id: FramebufferID,
}

pub enum BufferUsage {
	Static,
	Dynamic,
}

impl From<BufferUsage> for u32 {
	fn from(buffer_usage: BufferUsage) -> u32 {
		match buffer_usage {
			BufferUsage::Static => glow::STATIC_DRAW,
			BufferUsage::Dynamic => glow::DYNAMIC_DRAW,
		}
	}
}

