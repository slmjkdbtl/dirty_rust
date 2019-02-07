// wengwengweng

#![allow(dead_code)]

use std::ptr;
use std::mem;
use std::ffi::CString;
use std::any::TypeId;

use gl::types::*;

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

bind_enum!(BufferUsage(GLenum) {

	Static => gl::STATIC_DRAW,
	Dynamic => gl::DYNAMIC_DRAW,
	Stream => gl::STREAM_DRAW,

});

bind_enum!(ShaderType(GLenum) {

	Vertex => gl::VERTEX_SHADER,
	Fragment => gl::FRAGMENT_SHADER,

});

bind_enum!(Filter(GLenum) {

	Nearest => gl::NEAREST,
	Linear => gl::LINEAR,

});

bind_enum!(DepthFunc(GLenum) {

	Never => gl::NEVER,
	Less => gl::LESS,
	Equal => gl::EQUAL,
	LessOrEqual => gl::LEQUAL,
	Greater => gl::GREATER,
	NotEqual => gl::NOTEQUAL,
	GreaterOrEqual => gl::GEQUAL,
	Always => gl::ALWAYS,

});

bind_enum!(BlendFac(GLenum) {

	Zero => gl::ZERO,
	One => gl::ONE,
	SourceColor => gl::SRC_COLOR,
	OneMinusSourceColor => gl::ONE_MINUS_SRC_COLOR,
	DestinationColor => gl::DST_COLOR,
	OneMinusDestinationColor => gl::ONE_MINUS_DST_COLOR,
	SourceAlpha => gl::SRC_ALPHA,
	OneMinusSourceAlpha => gl::ONE_MINUS_SRC_ALPHA,
	DestinationAlpha => gl::DST_ALPHA,
	OneMinusDestinationAlpha => gl::ONE_MINUS_DST_ALPHA,
	SourceAlphaSaturate => gl::SRC_ALPHA_SATURATE,
	ConstantColor => gl::CONSTANT_COLOR,
	OneMinusConstantColor => gl::ONE_MINUS_CONSTANT_COLOR,
	ConstantAlpha => gl::CONSTANT_ALPHA,
	OneMinusConstantAlpha => gl::ONE_MINUS_CONSTANT_ALPHA,

});

pub struct Mesh {

	vbuf: VertexBuffer,
	ibuf: IndexBuffer,
	count: usize,

}

impl Mesh {

	pub fn new<V: VertexLayout>(verts: &[V], indices: &[u32]) -> Self {

		let mut queue = Vec::new();

		for v in verts {
			v.push(&mut queue);
		}

		let vbuf = VertexBuffer::new::<V>(queue.len(), BufferUsage::Static);
		let ibuf = IndexBuffer::new(indices.len(), BufferUsage::Static);

		vbuf.data(&queue, 0);
		ibuf.data(indices, 0);

		return Self {

			vbuf: vbuf,
			ibuf: ibuf,
			count: indices.len(),

		};

	}

	pub fn draw(&self, tex: &Texture, program: &Program) {
		draw(&self.vbuf, &self.ibuf, program, tex, self.count);
	}

}

pub struct BatchedMesh {

	queue: Vec<f32>,
	max: usize,
	ibuf: IndexBuffer,
	vbuf: VertexBuffer,
	mesh_type: TypeId,
	vert_stride: usize,
	vert_count: usize,
	index_count: usize,

}

impl BatchedMesh {

	pub fn new<M: Shape + 'static>(max: usize) -> Self {

		let indices = M::indices();
		let vert_count = M::COUNT;
		let vert_stride = M::Vertex::STRIDE;
		let max_vertices = max * vert_stride * vert_count;
		let max_indices = max * indices.len();
		let queue: Vec<f32> = Vec::with_capacity(max_vertices);

		let indices_batch: Vec<u32> = indices
			.iter()
			.cycle()
			.take(max_indices)
			.enumerate()
			.map(|(i, vertex)| vertex + i as u32 / 6 * 4)
			.collect();

		let ibuf = IndexBuffer::new(max_indices, BufferUsage::Static);

		ibuf.data(&indices_batch, 0);

		let vbuf = VertexBuffer::new::<M::Vertex>(max_vertices, BufferUsage::Dynamic);

		return Self {

			queue: queue,
			max: max,
			ibuf: ibuf,
			vbuf: vbuf,
			mesh_type: TypeId::of::<M>(),
			index_count: indices.len(),
			vert_stride: vert_stride,
			vert_count: vert_count,

		};

	}

	pub fn push<M: Shape + 'static>(&mut self, mesh: M) {

		if TypeId::of::<M>() != self.mesh_type {
			panic!("invalid vertex");
		}

		if self.queue.len() >= self.queue.capacity() {
			self.queue.clear();
			panic!("reached maximum draw count");
		}

		mesh.push(&mut self.queue);

	}

	pub fn flush(&mut self, tex: &Texture, program: &Program) {

		if self.queue.is_empty() {
			return;
		}

		self.vbuf.data(&self.queue, 0);

		draw(
			&self.vbuf,
			&self.ibuf,
			&program,
			&tex,
			self.queue.len() / self.vert_stride / self.vert_count * self.index_count
		);

		self.queue.clear();

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

#[derive(PartialEq)]
pub struct VertexAttr {

	name: String,
	size: GLint,
	offset: usize,

}

impl VertexAttr {
	pub fn new(name: &str, size: GLint, offset: usize) -> Self {
		return Self {
			name: name.to_owned(),
			size: size,
			offset: offset,
		};
	}
}

#[derive(PartialEq)]
pub struct VertexBuffer {

	id: GLuint,
	size: usize,
	stride: usize,
	attrs: Vec<VertexAttr>,
	usage: BufferUsage,

}

impl VertexBuffer {

	pub fn new<V: VertexLayout>(
		size: usize,
		usage: BufferUsage) -> Self {

		unsafe {

			let mut id: GLuint = 0;
			let stride = V::STRIDE;

			gl::GenBuffers(1, &mut id);
			gl::BindBuffer(gl::ARRAY_BUFFER, id);

			gl::BufferData(
				gl::ARRAY_BUFFER,
				(size * mem::size_of::<GLfloat>()) as GLsizeiptr,
				ptr::null() as *const GLvoid,
				usage.into(),
			);

			gl::BindBuffer(gl::ARRAY_BUFFER, 0);

			return Self {
				id: id,
				size: size,
				stride: stride,
				usage: usage,
				attrs: V::attrs(),
			};

		}

	}

	pub fn data(
		&self,
		data: &[GLfloat],
		offset: usize) -> &Self {

		assert!(offset + data.len() <= self.size, "buffer data overflow");
		self.bind();

		unsafe {

			gl::BufferSubData(
				gl::ARRAY_BUFFER,
				(offset * mem::size_of::<GLfloat>()) as GLsizeiptr,
				(data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
				data.as_ptr() as *const GLvoid,
			);

		}

		self.unbind();

		return self;

	}

	fn bind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
		}

		return self;

	}

	fn unbind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}

		return self;

	}

}

impl Drop for VertexBuffer {

	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &self.id);
		}
	}

}

#[derive(PartialEq)]
pub struct IndexBuffer {

	id: GLuint,
	size: usize,

}

impl IndexBuffer {

	pub fn new(
		size: usize,
		usage: BufferUsage) -> Self {

		unsafe {

			let mut id: GLuint = 0;

			gl::GenBuffers(1, &mut id);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);

			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(size * mem::size_of::<GLuint>()) as GLsizeiptr,
				ptr::null() as *const GLvoid,
				usage.into(),
			);

			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

			return Self {
				id: id,
				size: size,
			};

		}

	}

	pub fn data(
		&self,
		data: &[GLuint],
		offset: usize) -> &Self {

		assert!(offset + data.len() <= self.size, "buffer data overflow");
		self.bind();

		unsafe {

			gl::BufferSubData(
				gl::ELEMENT_ARRAY_BUFFER,
				(offset * mem::size_of::<GLuint>()) as GLsizeiptr,
				(data.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
				data.as_ptr() as *const GLvoid,
			);

		}

		self.unbind();

		return self;

	}

	fn bind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
		}

		return self;

	}

	fn unbind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
		}

		return self;

	}

}

impl Drop for IndexBuffer {

	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &self.id);
		}
	}

}

#[derive(PartialEq)]
pub struct Texture {

	id: GLuint,
	pub width: u32,
	pub height: u32,

}

impl Texture {

	/// create an empty texture with width and height
	pub fn new(
		width: u32,
		height: u32) -> Self {

		unsafe {

			let mut id: GLuint = 0;

			gl::GenTextures(1, &mut id);
			gl::BindTexture(gl::TEXTURE_2D, id);

			gl::TexImage2D(

				gl::TEXTURE_2D,
				0,
				gl::RGBA as GLint,
				width as GLint,
				height as GLint,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				ptr::null(),

			);

			gl::BindTexture(gl::TEXTURE_2D, 0);

			return Self {

				id: id,
				width: width,
				height: height,

			};

		}

	}

	pub fn data(
		&self,
		pixels: &[u8]) -> &Self {

		self.bind();

		unsafe {

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
			gl::GenerateMipmap(gl::TEXTURE_2D);

			gl::TexSubImage2D(
				gl::TEXTURE_2D,
				0,
				0,
				0,
				self.width as GLint,
				self.height as GLint,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				pixels.as_ptr() as *const GLvoid
			);

		}

		self.unbind();

		return self;

	}

	pub fn set_filter(&self, f: Filter) -> &Self {

		self.bind();

		let f: u32 = f.into();

		unsafe {
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, f as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, f as GLint);
		}

		self.unbind();

		return self;

	}

	fn bind(&self) -> &Self {

		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}

		return self;

	}

	fn unbind(&self) -> &Self {

		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}

		return self;

	}

	pub fn get_data(&self) -> Vec<u8> {

		let size = (self.width * self.height * 4) as usize;

		if size == 0 || self.id == 0 {
			return Vec::new();
		} else {

			let mut pixels = vec![0.0 as u8; (self.width * self.height * 4) as usize];

			self.bind();

			unsafe {

				gl::GetTexImage(
					gl::TEXTURE_2D,
					0,
					gl::RGBA,
					gl::UNSIGNED_BYTE,
					pixels.as_mut_ptr() as *mut GLvoid,
				);

			}

			self.unbind();

			return pixels;

		}

	}

}

impl Drop for Texture {

	fn drop(&mut self) {
		unsafe {
			gl::DeleteTextures(1, &self.id);
		}
	}

}

#[derive(PartialEq)]
pub struct Framebuffer {

	id: GLuint,

}

impl Framebuffer {

	/// create a frame buffer from width and height
	pub fn new() -> Self {

		unsafe {

			let mut id: GLuint = 0;

			gl::GenFramebuffers(1, &mut id);

			return Self {
				id: id,
			};

		}

	}

	pub fn attach(&self, tex: &Texture) -> &Self {

		self.bind();

		unsafe {

			gl::DrawBuffer(gl::COLOR_ATTACHMENT0);

			gl::FramebufferTexture2D(
				gl::FRAMEBUFFER,
				gl::COLOR_ATTACHMENT0,
				gl::TEXTURE_2D,
				tex.id,
				0
			);

		}

		clear(true, true, true);
		self.unbind();

		return self;

	}

	fn bind(&self) -> &Self {

		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
		}

		return self;

	}

	fn unbind(&self) -> &Self {

		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		}

		return self;

	}

}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteFramebuffers(1, &self.id);
		}
	}
}

#[derive(PartialEq)]
pub struct Program {
	id: GLuint,
}

impl Program {

	pub fn new(
		vs_src: &str,
		fs_src: &str) -> Self {

		unsafe {

			let vs: GLuint = compile_shader(ShaderType::Vertex, vs_src);
			let fs: GLuint = compile_shader(ShaderType::Fragment, fs_src);
			let id: GLuint = gl::CreateProgram();

			gl::AttachShader(id, vs);
			gl::AttachShader(id, fs);
			gl::LinkProgram(id);

			return Self {
				id: id
			};

		}

	}

	fn bind(&self) -> &Self {

		unsafe {
			gl::UseProgram(self.id);
		}

		return self;

	}

	fn unbind(&self) -> &Self {

		unsafe {
			gl::UseProgram(0);
		}

		return self;

	}

	pub fn uniform_f1(
		&self,
		name: &str,
		f: f32) -> &Self {

		self.bind();

		unsafe {
			gl::Uniform1f(gl::GetUniformLocation(self.id, cstr(name).as_ptr()), f);
		}

		self.unbind();

		return self;

	}

	pub fn uniform_f2(
		&self,
		name: &str,
		f1: f32,
		f2: f32) -> &Self {

		self.bind();

		unsafe {
			gl::Uniform2f(
				gl::GetUniformLocation(self.id, cstr(name).as_ptr()),
				f1,
				f2,
			);
		}

		self.unbind();

		return self;

	}

	pub fn uniform_f3(
		&self,
		name: &str,
		f1: f32,
		f2: f32,
		f3: f32) -> &Self {

		self.bind();

		unsafe {
			gl::Uniform3f(
				gl::GetUniformLocation(self.id, cstr(name).as_ptr()),
				f1,
				f2,
				f3,
			);
		}

		self.unbind();

		return self;

	}

	pub fn uniform_f4(
		&self,
		name: &str,
		f1: f32,
		f2: f32,
		f3: f32,
		f4: f32) -> &Self {

		self.bind();

		unsafe {
			gl::Uniform4f(
				gl::GetUniformLocation(self.id, cstr(name).as_ptr()),
				f1,
				f2,
				f3,
				f4,
			);
		}

		self.unbind();

		return self;

	}

	pub fn uniform_mat4(
		&self,
		name: &str,
		value: [[f32; 4]; 4]) -> &Self {

		self.bind();

		unsafe {
			gl::UniformMatrix4fv(
				gl::GetUniformLocation(self.id, cstr(name).as_ptr()),
				1,
				gl::FALSE,
				&value[0][0]
			);
		}

		self.unbind();

		return self;

	}

}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}
}

fn cstr(name: &str) -> CString {
	return CString::new(name).expect("failed to parse cstring");
}

fn compile_shader(
	shader_type: ShaderType,
	src: &str) -> GLuint {

	unsafe {

		let id: GLuint = gl::CreateShader(shader_type.into());
		let src_cstr = cstr(src);

		gl::ShaderSource(id, 1, &src_cstr.as_ptr(), ptr::null());
		gl::CompileShader(id);

		let mut status: GLint = gl::FALSE as GLint;

		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);

		if status != (gl::TRUE as GLint) {

			let mut log_length: GLint = mem::uninitialized();

			gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length);

			let mut log: Vec<u8> = Vec::with_capacity(log_length as usize);

			gl::GetShaderInfoLog(
				id,
				log_length,
				&mut log_length,
				log.as_mut_ptr() as *mut GLchar
			);

			log.set_len(log_length as usize);
			panic!("{}", String::from_utf8(log).expect("failed to get error log"));

		}

		return id;

	}

}

pub fn set_depth(d: DepthFunc) {

	unsafe {

		gl::Enable(gl::DEPTH_TEST);
		gl::DepthFunc(d.into());

	}
}

pub fn set_blend(sfac: BlendFac, dfac: BlendFac) {

	unsafe {

		gl::Enable(gl::BLEND);
		gl::BlendFunc(sfac.into(), dfac.into());

	}

}

pub fn set_stencil() {
	unsafe {
		gl::Enable(gl::STENCIL_TEST);
	}
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
	unsafe {
		gl::ClearColor(r, g, b, a);
	}
}

pub fn clear(color: bool, depth: bool, stencil: bool) {

	unsafe {

		let mut flags: u32 = 0;

		if color {
			flags |= gl::COLOR_BUFFER_BIT;
		}

		if depth {
			flags |= gl::DEPTH_BUFFER_BIT;
		}

		if stencil {
			flags |= gl::STENCIL_BUFFER_BIT;
		}

		gl::Clear(flags);

	}

}

pub fn set_framebuffer(fb: &Framebuffer) {
	fb.bind();
}

pub fn unset_framebuffer(fb: &Framebuffer) {
	fb.unbind();
}

pub fn draw(
	vbuf: &VertexBuffer,
	ibuf: &IndexBuffer,
	program: &Program,
	tex: &Texture,
	count: usize) {

	unsafe {

		program.bind();
		vbuf.bind();
		ibuf.bind();
		tex.bind();

		for attr in &vbuf.attrs {

			let index = gl::GetAttribLocation(program.id, cstr(&attr.name).as_ptr()) as u32;

			gl::VertexAttribPointer(
				index,
				attr.size,
				gl::FLOAT,
				gl::FALSE,
				(vbuf.stride * mem::size_of::<GLfloat>()) as GLsizei,
				(attr.offset * mem::size_of::<GLfloat>()) as *const GLvoid
			);

			gl::EnableVertexAttribArray(index);

		}

		gl::DrawElements(
			gl::TRIANGLES,
			count as GLsizei,
			gl::UNSIGNED_INT,
			ptr::null(),
		);

		vbuf.unbind();
		ibuf.unbind();
		tex.unbind();
		program.unbind();

	}

}

