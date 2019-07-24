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

use crate::Error;
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

	pub fn draw_elements(
		&mut self,
		vbuf: &VertexBuffer,
		ibuf: &IndexBuffer,
		tex: &Texture,
		program: &Program,
		count: i32,
	) {

		unsafe {

			vbuf.bind();
			ibuf.bind();
			tex.bind();
			program.bind();

			self.gl.draw_elements(glow::TRIANGLES, count, glow::UNSIGNED_INT, 0);

			vbuf.unbind();
			ibuf.unbind();
			tex.unbind();
			program.unbind();

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

	ctx: Rc<GLCtx>,
	id: BufferID,
	count: usize,
	stride: usize,

}

impl VertexBuffer {

	pub fn new(device: &Device, count: usize, stride: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let ctx = device.gl.clone();
			let id = ctx.create_texture()?;

			let buf = Self {
				ctx: ctx,
				id: id,
				count: count,
				stride: stride,
			};

			buf.bind();

			buf.ctx.buffer_data_size(
				glow::ARRAY_BUFFER,
				(count * mem::size_of::<f32>()) as i32,
				usage.into(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn bind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));
		}
	}

	pub fn unbind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ARRAY_BUFFER, None);
		}
	}

	pub fn data(&self, data: &[f32], offset: usize) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(offset * mem::size_of::<f32>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

}

impl Drop for VertexBuffer {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_buffer(self.id);
		}
	}
}

pub struct IndexBuffer {

	ctx: Rc<GLCtx>,
	id: BufferID,
	count: usize,

}

impl IndexBuffer {

	pub fn new(device: &Device, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let ctx = device.gl.clone();
			let id = ctx.create_buffer()?;

			let buf = Self {
				ctx: ctx,
				id: id,
				count: count,
			};

			buf.bind();

			buf.ctx.buffer_data_size(
				glow::ELEMENT_ARRAY_BUFFER,
				(count * mem::size_of::<f32>()) as i32,
				usage.into(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn bind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.id));
		}
	}

	pub fn unbind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
		}
	}

	pub fn data(&self, data: &[f32], offset: usize) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ELEMENT_ARRAY_BUFFER,
				(offset * mem::size_of::<f32>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

}

impl Drop for IndexBuffer {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_buffer(self.id);
		}
	}
}

pub struct Texture {
	ctx: Rc<GLCtx>,
	id: TextureID,
	width: i32,
	height: i32,
}

impl Texture {

	pub fn new(device: &Device, width: i32, height: i32) -> Result<Self> {

		unsafe {

			let ctx = device.gl.clone();
			let id = ctx.create_texture()?;

			let tex = Self {
				ctx: ctx,
				id: id,
				width: width,
				height: height,
			};

			tex.bind();

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				glow::REPEAT as i32
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				glow::REPEAT as i32
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				FilterMode::Nearest.into(),
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				FilterMode::Nearest.into(),
			);

			tex.ctx.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				width,
				height,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				None,
			);

			tex.unbind();

			return Ok(tex);

		}

	}

	fn bind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_2D, Some(self.id));
		}
	}

	fn unbind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_2D, None);
		}
	}

	pub fn data(&self, data: &[u8]) {

		unsafe {

			self.bind();

			self.ctx.tex_sub_image_2d_u8_slice(
				glow::TEXTURE_2D,
				0,
				0,
				0,
				self.width,
				self.height,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(data),
			);

			self.unbind();

		}

	}

}

impl Drop for Texture {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_texture(self.id);
		}
	}
}

pub struct Program {
	ctx: Rc<GLCtx>,
	id: ProgramID,
}

impl Program {

	pub fn new(device: &Device, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let ctx = device.gl.clone();
			let program_id = ctx.create_program()?;

			let vert_id = ctx.create_shader(glow::VERTEX_SHADER)?;

			ctx.shader_source(vert_id, vert_src);
			ctx.compile_shader(vert_id);
			ctx.attach_shader(program_id, vert_id);

			if !ctx.get_shader_compile_status(vert_id) {
				return Err(Error::OpenGL(ctx.get_shader_info_log(vert_id)));
			}

			let frag_id = ctx.create_shader(glow::FRAGMENT_SHADER)?;

			ctx.shader_source(frag_id, frag_src);
			ctx.compile_shader(frag_id);
			ctx.attach_shader(program_id, frag_id);

			if !ctx.get_shader_compile_status(frag_id) {
				return Err(Error::OpenGL(ctx.get_shader_info_log(frag_id)));
			}

			ctx.link_program(program_id);

			if !ctx.get_program_link_status(program_id) {
				return Err(Error::OpenGL(ctx.get_program_info_log(program_id)));
			}

			ctx.delete_shader(vert_id);
			ctx.delete_shader(frag_id);

			let program = Self {
				ctx: ctx,
				id: program_id,
			};

			return Ok(program);

		}

	}

	pub fn send<T: UniformValue>(&self, name: &str, value: T) {

		unsafe {

			self.bind();
			value.send(&self.ctx, self.ctx.get_uniform_location(self.id, name));
			self.unbind();

		}

	}

	pub fn bind(&self) {
		unsafe {
			self.ctx.use_program(Some(self.id));
		}
	}

	pub fn unbind(&self) {
		unsafe {
			self.ctx.use_program(None);
		}
	}

}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_program(self.id);
		}
	}
}

pub struct Framebuffer {

	ctx: Rc<GLCtx>,
	id: FramebufferID,
	tex: Texture,

}

impl Framebuffer {

	pub fn new(device: &Device, width: i32, height: i32) -> Result<Self> {

		unsafe {

			let ctx = device.gl.clone();
			let id = ctx.create_framebuffer()?;
			let tex = Texture::new(device, width, height)?;

			let fbuf = Self {
				ctx: ctx,
				id: id,
				tex: tex,
			};

			fbuf.bind();

			fbuf.ctx.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(fbuf.tex.id),
				0,
			);

			fbuf.unbind();

			return Ok(fbuf);

		}
	}

	pub fn bind(&self) {
		unsafe {
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
		}
	}

	pub fn unbind(&self) {
		unsafe {
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
	}

}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_framebuffer(self.id);
		}
	}
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum FilterMode {
	Linear,
	Nearest,
}

impl From<FilterMode> for i32 {
	fn from(filter_mode: FilterMode) -> i32 {
		match filter_mode {
			FilterMode::Nearest => glow::NEAREST as i32,
			FilterMode::Linear => glow::LINEAR as i32,
		}
	}
}

pub enum ShaderType {
	Vertex,
	Fragment,
}

pub trait UniformValue {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>);
}

impl UniformValue for i32 {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_1_i32(loc, *self);
	}
}

impl UniformValue for f32 {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_1_f32(loc, *self);
	}
}

impl UniformValue for [f32; 2] {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_2_f32(loc, self[0], self[1]);
	}
}

impl UniformValue for [i32; 2] {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_2_i32(loc, self[0], self[1]);
	}
}

impl UniformValue for [f32; 3] {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_3_f32(loc, self[0], self[1], self[2]);
	}
}

impl UniformValue for [i32; 3] {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_3_i32(loc, self[0], self[1], self[2]);
	}
}

impl UniformValue for [f32; 4] {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_4_f32(loc, self[0], self[1], self[2], self[3]);
	}
}

impl UniformValue for [i32; 4] {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_4_i32(loc, self[0], self[1], self[2], self[3]);
	}
}

impl UniformValue for Vec2 {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_2_f32(loc, self.x, self.y);
	}
}

impl UniformValue for Vec3 {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_3_f32(loc, self.x, self.y, self.z);
	}
}

impl UniformValue for Vec4 {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_4_f32(loc, self.x, self.y, self.z, self.w);
	}
}

impl UniformValue for Color {
	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
		ctx.uniform_4_f32(loc, self.r, self.g, self.b, self.a);
	}
}

// impl UniformValue for Mat4 {
// 	unsafe fn send(&self, ctx: &GLCtx, loc: Option<u32>) {
// 		ctx.uniform_4_f32_slice(loc, self);
// 	}
// }

