// wengwengweng

use std::mem;
use std::rc::Rc;
use std::marker::PhantomData;

use glow::Context;

use crate::Error;
use crate::Result;
use crate::math::*;

type GLCtx = glow::native::Context;
type BufferID = <GLCtx as Context>::Buffer;
type ProgramID = <GLCtx as Context>::Program;
type TextureID = <GLCtx as Context>::Texture;
type FramebufferID = <GLCtx as Context>::Framebuffer;
type VertexArrayID = <GLCtx as Context>::VertexArray;

pub struct Device {
	ctx: Rc<GLCtx>,
}

impl Device {

	pub fn from_loader<F: FnMut(&str) -> *const std::os::raw::c_void>(f: F) -> Self {

		return Self {
			ctx: Rc::new(GLCtx::from_loader_function(f)),
		};

	}

	pub fn enable(&self, cap: Capability) {
		unsafe {
			self.ctx.enable(cap.into());
		}
	}

	pub fn disable(&self, cap: Capability) {
		unsafe {
			self.ctx.disable(cap.into());
		}
	}

	pub fn blend_func(&self, src: BlendFac, dest: BlendFac) {
		unsafe {
			self.ctx.blend_func(src.into(), dest.into());
		}
	}

	pub fn blend_func_sep(&self, src_rgb: BlendFac, dest_rgb: BlendFac, src_a: BlendFac, dest_a: BlendFac) {
		unsafe {
			self.ctx.blend_func_separate(src_rgb.into(), dest_rgb.into(), src_a.into(), dest_a.into());
		}
	}

	#[cfg(feature="gl3")]
	pub fn draw(&self, vao: &VertexArray, ibuf: &IndexBuffer, program: &Program, count: u32) {

		vao.bind();
		ibuf.bind();
		program.bind();

		unsafe {
			self.ctx.draw_elements(DrawMode::Triangle.into(), count as i32, glow::UNSIGNED_INT, 0);
		}

		program.unbind();
		ibuf.unbind();
		vao.unbind();

	}

	#[cfg(not(feature="gl3"))]
	pub fn draw<V: VertexLayout>(&self, vbuf: &VertexBuffer<V>, ibuf: &IndexBuffer, program: &Program, count: u32) {

		program.bind();
		vbuf.bind();
		vbuf.bind_attrs(program);
		ibuf.bind();

		unsafe {
			self.ctx.draw_elements(DrawMode::Triangle.into(), count as i32, glow::UNSIGNED_INT, 0);
		}

		ibuf.unbind();
		vbuf.unbind();
		program.unbind();

	}

	pub fn get_error(&self) -> Result<()> {

		unsafe {

			use Error::OpenGL;

			return match self.ctx.get_error() {
				glow::NO_ERROR => Ok(()),
				glow::INVALID_ENUM => Err(OpenGL("INVALID_ENUM".to_owned())),
				glow::INVALID_VALUE => Err(OpenGL("INVALID_VALUE".to_owned())),
				glow::INVALID_OPERATION => Err(OpenGL("INVALID_OPERATION".to_owned())),
				glow::STACK_OVERFLOW => Err(OpenGL("STACK_OVERFLOW".to_owned())),
				glow::STACK_UNDERFLOW => Err(OpenGL("STACK_UNDERFLOW".to_owned())),
				glow::OUT_OF_MEMORY => Err(OpenGL("OUT_OF_MEMORY".to_owned())),
				glow::INVALID_FRAMEBUFFER_OPERATION => Err(OpenGL("INVALID_FRAMEBUFFER_OPERATION".to_owned())),
				_ => Err(OpenGL("UNKNOWN".to_owned())),
			};

		}

	}

	pub fn clear_color(&self, c: Color) {
		unsafe {
			self.ctx.clear_color(c.r, c.g, c.b, c.a);
		}
	}

	pub fn clear(&self) {
		unsafe {
			self.ctx.clear(glow::COLOR_BUFFER_BIT);
		}
	}

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.ctx.viewport(x, y, width, height);
        }
    }

}

pub struct Renderer<S: Shape> {
	#[cfg(feature="gl3")]
	vao: VertexArray,
	#[cfg(not(feature="gl3"))]
	vbuf: VertexBuffer<S::Vertex>,
	ibuf: IndexBuffer,
	shape: PhantomData<S>,
}

impl<S: Shape> Renderer<S> {

	pub fn new(device: &Device, s: S) -> Result<Self> {

		let indices = S::indices();
		let vbuf = VertexBuffer::<S::Vertex>::new(&device, S::COUNT, BufferUsage::Static)?;
		let ibuf = IndexBuffer::new(&device, indices.len(), BufferUsage::Static)?;

		ibuf.data(0, &indices);

		let mut queue = vec![];

		s.push(&mut queue);
		vbuf.data(0, &queue);

		#[cfg(feature="gl3")]
		let vao = VertexArray::new(&device)?;
		#[cfg(feature="gl3")]
		vao.attr(&vbuf);

		return Ok(Self {
			#[cfg(feature="gl3")]
			vao: vao,
			#[cfg(not(feature="gl3"))]
			vbuf: vbuf,
			ibuf: ibuf,
			shape: PhantomData,
		});

	}

	pub fn draw(&mut self, device: &Device, program: &Program) {

		#[cfg(feature="gl3")]
		device.draw(
			&self.vao,
			&self.ibuf,
			&program,
			S::indices().len() as u32
		);

		#[cfg(not(feature="gl3"))]
		device.draw(
			&self.vbuf,
			&self.ibuf,
			&program,
			S::indices().len() as u32
		);

	}

}

pub struct BatchedRenderer<S: Shape> {

	vbuf: VertexBuffer<S::Vertex>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	queue: Vec<f32>,
	shape: PhantomData<S>,

}

impl<S: Shape> BatchedRenderer<S> {

	pub fn new(device: &Device, max: usize) -> Result<Self> {

		let indices = S::indices();
		let vert_count = S::COUNT;
		let vert_stride = S::Vertex::STRIDE;
		let max_vertices = max * vert_stride * vert_count;
		let max_indices = max * indices.len();

		let indices_batch: Vec<u32> = indices
			.iter()
			.cycle()
			.take(max * indices.len())
			.enumerate()
			.map(|(i, vertex)| vertex + i as u32 / 6 * 4)
			.collect();

		let vbuf = VertexBuffer::new(&device, S::COUNT * max, BufferUsage::Dynamic)?;
		let ibuf = IndexBuffer::new(&device, max_indices, BufferUsage::Static)?;

		ibuf.data(0, &indices_batch);

		let queue = Vec::with_capacity(max_vertices);

		#[cfg(feature="gl3")]
		let vao = VertexArray::new(&device)?;
		#[cfg(feature="gl3")]
		vao.attr(&vbuf);

		return Ok(Self {
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			queue: queue,
			shape: PhantomData,
		});

	}

	pub fn push(&mut self, mesh: S) -> Result<()> {

		if self.queue.len() >= self.queue.capacity() {
			self.queue.clear();
			return Err(Error::MaxDraw);
		}

		mesh.push(&mut self.queue);

		return Ok(());

	}

	pub fn flush(&mut self, device: &Device, program: &Program) {

		if self.empty() {
			return;
		}

		self.vbuf.data(0, &self.queue);

		#[cfg(feature="gl3")]
		device.draw(
			&self.vao,
			&self.ibuf,
			&program,
			(self.queue.len() * S::indices().len() / S::Vertex::STRIDE / S::COUNT) as u32
		);

		#[cfg(not(feature="gl3"))]
		device.draw(
			&self.vbuf,
			&self.ibuf,
			&program,
			(self.queue.len() * S::indices().len() / S::Vertex::STRIDE / S::COUNT) as u32
		);

		self.queue.clear();

	}

	pub fn empty(&self) -> bool {
		return self.queue.is_empty();
	}

}

pub trait Shape {

	type Vertex: VertexLayout;
	const COUNT: usize;
	fn push(&self, queue: &mut Vec<f32>);
	fn indices() -> Vec<u32>;

}

pub trait VertexLayout {

	const STRIDE: usize;
	fn push(&self, queue: &mut Vec<f32>);
	fn attrs() -> Vec<VertexAttr>;

}

pub struct VertexAttr {

	name: String,
	size: i32,
	offset: usize,

}

impl VertexAttr {
	pub fn new(name: &str, size: i32, offset: usize) -> Self {
		return Self {
			name: name.to_owned(),
			size: size,
			offset: offset,
		};
	}
}

pub struct VertexArray {
	ctx: Rc<GLCtx>,
	id: VertexArrayID,
}

impl VertexArray {

	pub fn new(device: &Device) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_vertex_array()?;

			let buf = Self {
				ctx: ctx,
				id: id,
			};

			return Ok(buf);

		}

	}

	pub fn bind(&self) {
		unsafe {
			self.ctx.bind_vertex_array(Some(self.id));
		}
	}

	pub fn unbind(&self) {
		unsafe {
			self.ctx.bind_vertex_array(None);
		}
	}

	pub fn attr<V: VertexLayout>(&self, vbuf: &VertexBuffer<V>) {

		unsafe {

			self.bind();
			vbuf.bind();

			for (i, attr) in V::attrs().iter().enumerate() {

				self.ctx.vertex_attrib_pointer_f32(
					i as u32,
					attr.size,
					glow::FLOAT,
					false,
					(V::STRIDE * mem::size_of::<f32>()) as i32,
					(attr.offset * mem::size_of::<f32>()) as i32,
				);

				self.ctx.enable_vertex_attrib_array(i as u32);

			}

			vbuf.unbind();
			self.unbind();

		}

	}

}

impl Drop for VertexArray {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_vertex_array(self.id);
		}
	}
}

impl PartialEq for VertexArray {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

pub struct VertexBuffer<V: VertexLayout> {

	ctx: Rc<GLCtx>,
	id: BufferID,
	stride: usize,
	attrs: Vec<VertexAttr>,
	layout: PhantomData<V>,

}

impl<V: VertexLayout> VertexBuffer<V> {

	pub fn new(device: &Device, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_buffer()?;

			let buf = Self {
				ctx: ctx,
				id: id,
				stride: V::STRIDE,
				attrs: V::attrs(),
				layout: PhantomData,
			};

			buf.bind();

			buf.ctx.buffer_data_size(
				glow::ARRAY_BUFFER,
				(count * V::STRIDE * mem::size_of::<f32>()) as i32,
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

	pub fn bind_attrs(&self, program: &Program) {

		unsafe {

			for attr in &self.attrs {

				let index = self.ctx.get_attrib_location(program.id, &attr.name) as u32;

				self.ctx.vertex_attrib_pointer_f32(
					index,
					attr.size,
					glow::FLOAT,
					false,
					(self.stride * mem::size_of::<f32>()) as i32,
					(attr.offset * mem::size_of::<f32>()) as i32,
				);

				self.ctx.enable_vertex_attrib_array(index);

			}

		}

	}

	// TODO: change this to take V
	pub fn data(&self, offset: usize, data: &[f32]) {

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

impl<V: VertexLayout> Drop for VertexBuffer<V> {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_buffer(self.id);
		}
	}
}

impl<V: VertexLayout> PartialEq for VertexBuffer<V> {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

pub struct IndexBuffer {

	ctx: Rc<GLCtx>,
	id: BufferID,

}

impl IndexBuffer {

	pub fn new(device: &Device, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_buffer()?;

			let buf = Self {
				ctx: ctx,
				id: id,
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

	pub fn data(&self, offset: usize, data: &[u32]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ELEMENT_ARRAY_BUFFER,
				(offset * mem::size_of::<u32>()) as i32,
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

impl PartialEq for IndexBuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

pub struct Texture {
	ctx: Rc<GLCtx>,
	id: TextureID,
	pub width: i32,
	pub height: i32,
}

impl Texture {

	pub fn new(device: &Device, width: i32, height: i32) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
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

	pub fn bind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_2D, Some(self.id));
		}
	}

	pub fn unbind(&self) {
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

	pub fn get_data(&self) -> Vec<u8> {

		let size = (self.width * self.height * 4) as usize;
		let pixels = vec![0.0 as u8; size];

		self.bind();

		unsafe {

			self.ctx.get_tex_image_u8_slice(
				glow::TEXTURE_2D,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(&pixels),
			);

		}

		self.unbind();

		return pixels;

	}

}

impl Drop for Texture {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_texture(self.id);
		}
	}
}

impl PartialEq for Texture {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

pub struct Program {
	ctx: Rc<GLCtx>,
	id: ProgramID,
}

impl Program {

	pub fn new(device: &Device, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let program_id = ctx.create_program()?;

			let vert_id = ctx.create_shader(ShaderType::Vertex.into())?;

			ctx.shader_source(vert_id, vert_src);
			ctx.compile_shader(vert_id);
			ctx.attach_shader(program_id, vert_id);

			if !ctx.get_shader_compile_status(vert_id) {
				return Err(Error::OpenGL(ctx.get_shader_info_log(vert_id)));
			}

			let frag_id = ctx.create_shader(ShaderType::Fragment.into())?;

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

	pub fn send(&self, name: &str, value: impl UniformValue) {

		unsafe {

			self.bind();

			use UniformType::*;

			let loc = self.ctx.get_uniform_location(self.id, name);

			match value.get() {
				F1(f) => self.ctx.uniform_1_f32(loc, f),
				F2(f1, f2) => self.ctx.uniform_2_f32(loc, f1, f2),
				F3(f1, f2, f3) => self.ctx.uniform_3_f32(loc, f1, f2, f3),
				F4(f1, f2, f3, f4) => self.ctx.uniform_4_f32(loc, f1, f2, f3, f4),
				Mat4(a) => self.ctx.uniform_matrix_4_f32_slice(loc, false, &a),
			}

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

impl PartialEq for Program {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

pub struct Framebuffer {

	ctx: Rc<GLCtx>,
	id: FramebufferID,

}

impl Framebuffer {

	pub fn new(device: &Device, tex: &Texture) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_framebuffer()?;

			let fbuf = Self {
				ctx: ctx,
				id: id,
			};

			fbuf.bind();

			fbuf.ctx.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(tex.id),
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

impl PartialEq for Framebuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

macro_rules! bind_enum {

	($name:ident($type:ty) { $($member:ident => $dest:expr),+$(,)? }) => {

		#[allow(missing_docs)]
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub enum $name {
			$($member,)+
		}

		impl From<$name> for $type {

			fn from(usage: $name) -> $type {

				match usage {
					$($name::$member => $dest,)+
				}

			}

		}

	};

}

bind_enum!(BufferUsage(u32) {
	Static => glow::STATIC_DRAW,
	Dynamic => glow::DYNAMIC_DRAW,
	Stream => glow::STREAM_DRAW,
});

bind_enum!(FilterMode(i32) {
	Nearest => glow::NEAREST as i32,
	Linear => glow::LINEAR as i32,
});

bind_enum!(Capability(u32) {
	Blend => glow::BLEND,
	CullFace => glow::CULL_FACE,
	DepthTest => glow::DEPTH_TEST,
	StencilTest => glow::STENCIL_TEST,
	ScissorTest => glow::SCISSOR_TEST,
});

bind_enum!(BlendFac(u32) {
	Zero => glow::ZERO,
	One => glow::ONE,
	SourceColor => glow::SRC_COLOR,
	OneMinusSourceColor => glow::ONE_MINUS_SRC_COLOR,
	DestinationColor => glow::DST_COLOR,
	OneMinusDestinationColor => glow::ONE_MINUS_DST_COLOR,
	SourceAlpha => glow::SRC_ALPHA,
	OneMinusSourceAlpha => glow::ONE_MINUS_SRC_ALPHA,
	DestinationAlpha => glow::DST_ALPHA,
	OneMinusDestinationAlpha => glow::ONE_MINUS_DST_ALPHA,
	SourceAlphaSaturate => glow::SRC_ALPHA_SATURATE,
	ConstantColor => glow::CONSTANT_COLOR,
	OneMinusConstantColor => glow::ONE_MINUS_CONSTANT_COLOR,
	ConstantAlpha => glow::CONSTANT_ALPHA,
	OneMinusConstantAlpha => glow::ONE_MINUS_CONSTANT_ALPHA,
});

bind_enum!(DrawMode(u32) {
	Point => glow::POINT,
	Line => glow::LINE,
	Triangle => glow::TRIANGLES,
	LineStrip => glow::LINE_STRIP,
	TriangleFan => glow::TRIANGLE_FAN,
	TriangleStrip => glow::TRIANGLE_STRIP,
});

bind_enum!(ShaderType(u32) {
	Vertex => glow::VERTEX_SHADER,
	Fragment => glow::FRAGMENT_SHADER,
});

bind_enum!(DepthFunc(u32) {

	Never => glow::NEVER,
	Less => glow::LESS,
	Equal => glow::EQUAL,
	LessOrEqual => glow::LEQUAL,
	Greater => glow::GREATER,
	NotEqual => glow::NOTEQUAL,
	GreaterOrEqual => glow::GEQUAL,
	Always => glow::ALWAYS,

});

pub enum UniformType {
	F1(f32),
	F2(f32, f32),
	F3(f32, f32, f32),
	F4(f32, f32, f32, f32),
	Mat4([f32; 16]),
}

pub trait UniformValue {
	fn get(&self) -> UniformType;
}

impl UniformValue for f32 {
	fn get(&self) -> UniformType {
		return UniformType::F1(*self);
	}
}

impl UniformValue for [f32; 2] {
	fn get(&self) -> UniformType {
		return UniformType::F2(self[0], self[1]);
	}
}

impl UniformValue for [f32; 3] {
	fn get(&self) -> UniformType {
		return UniformType::F3(self[0], self[1], self[2]);
	}
}

impl UniformValue for [f32; 4] {
	fn get(&self) -> UniformType {
		return UniformType::F4(self[0], self[1], self[2], self[3]);
	}
}

impl UniformValue for Vec2 {
	fn get(&self) -> UniformType {
		return UniformType::F2(self.x, self.y);
	}
}

impl UniformValue for Vec3 {
	fn get(&self) -> UniformType {
		return UniformType::F3(self.x, self.y, self.z);
	}
}

impl UniformValue for Vec4 {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.x, self.y, self.z, self.w);
	}
}

impl UniformValue for Color {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.r, self.g, self.b, self.a);
	}
}

impl UniformValue for Quad {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.x, self.y, self.w, self.h);
	}
}

impl UniformValue for Mat4 {
	fn get(&self) -> UniformType {
		return UniformType::Mat4(self.as_arr());
	}
}

