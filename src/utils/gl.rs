// wengwengweng

use std::ptr;
use std::mem;
use std::ffi::CString;
use std::ffi::c_void;

use gl::types::*;

use crate::*;

bind_enum!(BufferUsage(GLenum) {
	Static => gl::STATIC_DRAW,
	Dynamic => gl::DYNAMIC_DRAW,
});

bind_enum!(ShaderType(GLenum) {
	Vertex => gl::VERTEX_SHADER,
	Fragment => gl::FRAGMENT_SHADER,
});

bind_enum!(Filter(GLenum) {
	Nearest => gl::NEAREST,
	Linear => gl::LINEAR,
});

pub struct VertexBuffer {

	id: GLuint,
	size: usize,
	stride: usize,
	usage: BufferUsage,

}

impl VertexBuffer {

	pub fn new(
		size: usize,
		stride: usize,
		usage: BufferUsage) -> Self {

		unsafe {

			let mut id: GLuint = 0;

			gl::GenBuffers(1, &mut id);
			gl::BindBuffer(gl::ARRAY_BUFFER, id);

			gl::BufferData(
				gl::ARRAY_BUFFER,
				(size * mem::size_of::<GLfloat>()) as GLsizeiptr,
				ptr::null() as *const GLvoid,
				usage.into(),
			);

			return Self {
				id: id,
				size: size,
				stride: stride,
				usage: usage,
			};

		}

	}

	pub fn clear(&self) -> &Self {

		unsafe {

			gl::BufferData(
				gl::ARRAY_BUFFER,
				(self.size * mem::size_of::<GLfloat>()) as GLsizeiptr,
				ptr::null() as *const GLvoid,
				self.usage.into(),
			);

		}

		return self;

	}

	pub fn data(
		&self,
		data: &[GLfloat],
		offset: usize) -> &Self {

		unsafe {

			self.bind();

			gl::BufferSubData(
				gl::ARRAY_BUFFER,
				(offset * mem::size_of::<GLfloat>()) as GLsizeiptr,
				(data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
				data.as_ptr() as *const GLvoid,
			);

		}

		return self;

	}

	pub fn attr(
		&self,
		index: GLuint,
		size: GLint,
		offset: usize) -> &Self {

		unsafe {

			self.bind();

			gl::VertexAttribPointer(
				index,
				size,
				gl::FLOAT,
				gl::FALSE,
				(self.stride * mem::size_of::<GLfloat>()) as GLsizei,
				(offset * mem::size_of::<GLfloat>()) as *const GLvoid
			);

			gl::EnableVertexAttribArray(index);

		}

		return self;

	}

	pub fn bind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
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

		unsafe {

			assert!(offset + data.len() <= self.size);

			self.bind();

			gl::BufferSubData(
				gl::ELEMENT_ARRAY_BUFFER,
				(offset * mem::size_of::<GLuint>()) as GLsizeiptr,
				(data.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
				data.as_ptr() as *const GLvoid,
			);

			return self;

		}

	}

	pub fn bind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
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

pub struct Texture {

	id: GLuint,
	width: u32,
	height: u32,

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
				gl::RGBA8 as GLint,
				width as GLint,
				height as GLint,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				ptr::null(),

			);

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

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
			gl::GenerateMipmap(gl::TEXTURE_2D);

			gl::TexSubImage2D(

				gl::TEXTURE_2D,
				0,
				gl::RGBA8 as GLint,
				self.width as GLint,
				self.height as GLint,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				pixels.as_ptr() as *const GLvoid

			);

		}

		return self;

	}

	pub fn bind(&self) -> &Self {

		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}

		return self;

	}

}

impl Drop for Texture {

	fn drop(&mut self) {
		unsafe {
			gl::DeleteTextures(1, &self.id);
		}
	}

}

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

	pub fn attach(
		&mut self,
		tex: &Texture) -> &Self {

		self.bind();

		unsafe {

			gl::FramebufferTexture2D(
				gl::FRAMEBUFFER,
				gl::COLOR_ATTACHMENT0,
				gl::TEXTURE_2D,
				tex.id,
				0,
			);

		}

		return self;

	}

	fn bind(&self) -> &Self {

		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
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

			return Self {
				id: id
			};

		}

	}

	pub fn attr(
		&self,
		index: GLuint,
		name: &str) -> &Self {

		unsafe {
			gl::BindAttribLocation(self.id, index, cstr(name).as_ptr());
		}

		return self;

	}

	pub fn bind(&self) -> &Self {

		unsafe {
			gl::UseProgram(self.id);
		}

		return self;

	}

	pub fn link(&self) -> &Self {

		unsafe {
			gl::LinkProgram(self.id);
		}

		return self;

	}

	pub fn uniform_color(&self, name: &str, c: Color) -> &Self {
		return self.uniform_vec4(name, vec4!(c.r, c.g, c.b, c.a));
	}

	pub fn uniform_rect(&self, name: &str, r: Rect) -> &Self {
		return self.uniform_vec4(name, vec4!(r.x, r.y, r.w, r.h));
	}

	pub fn uniform_vec4(
		&self,
		name: &str,
		v: Vec4) -> &Self {

		unsafe {
			gl::Uniform4f(
				gl::GetUniformLocation(self.id, cstr(name).as_ptr()),
				v.x,
				v.y,
				v.z,
				v.w,
			);
		}

		return self;

	}

	pub fn uniform_mat4(
		&self,
		name: &str,
		value: [[f32; 4]; 4]) -> &Self {

		unsafe {
			gl::UniformMatrix4fv(
				gl::GetUniformLocation(self.id, cstr(name).as_ptr()),
				1,
				gl::FALSE,
				&value[0][0]
			);
		}

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

pub fn clear() {
	unsafe {
		gl::Clear(gl::COLOR_BUFFER_BIT);
	}
}

pub fn draw(
	vbuf: &VertexBuffer,
	ibuf: &IndexBuffer,
	program: &Program,
// 	tex: &Texture,
	count: usize) {

	unsafe {

		program.bind();
		vbuf.bind();
		ibuf.bind();
// 		tex.bind();

		gl::DrawElements(
			gl::TRIANGLES,
			count as GLsizei,
			gl::UNSIGNED_INT,
			ptr::null(),
		);

	}

}

