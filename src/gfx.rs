// wengwengweng

use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::mem;

use crate::math;

struct Renderer2D {

	mesh: Mesh,
	program: Program,

}

impl Renderer2D {

	fn draw(&self) {
		self.program.bind();
		self.mesh.draw();
	}

}

fn make_renderer_2d() -> Renderer2D {

	let vertices: Vec<GLfloat> = vec![
		-0.5,  0.5,
		 0.5,  0.5,
		 0.5, -0.5,
		-0.5, -0.5,
	];

	let uv: Vec<GLfloat> = vec![
		0.0, 1.0,
		1.0, 1.0,
		1.0, 0.0,
		0.0, 0.0
	];

	let indices: Vec<GLuint> = vec![
		0, 1, 3,
		1, 2, 3,
	];

	let mut mesh = make_mesh();

	mesh.make_buf(&vertices).attr(0, 2);
	mesh.make_buf(&uv).attr(1, 2);
	mesh.make_index_buf(&indices);

	let program = make_program(
		include_str!("quad.vert").to_owned(),
		include_str!("quad.frag").to_owned()
	);

	program
		.attr(0, "pos")
		.attr(1, "uv")
		.link();

	return Renderer2D {
		mesh: mesh,
		program: program,
	};

}

pub struct Buffer {
	id: GLuint,
}

impl Buffer {

	pub fn bind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
		}

		return self;
	}

	pub fn unbind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}

		return self;
	}

	pub fn data(&self, data: &Vec<GLfloat>) -> &Self {

		unsafe {

			self.bind();

			gl::BufferData(
				gl::ARRAY_BUFFER,
				(data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
				mem::transmute(data.as_ptr()),
				gl::STATIC_DRAW
			);

			self.unbind();

		}

		return self;

	}

	pub fn attr(&self, attr_index: GLuint, buf_size: GLint) -> &Self {

		unsafe {

			self.bind();
			gl::VertexAttribPointer(attr_index, buf_size, gl::FLOAT, gl::FALSE, 0, ptr::null());
			gl::EnableVertexAttribArray(attr_index);
			self.unbind();

		}

		return self;

	}

}

pub struct IndexBuffer {
	id: GLuint,
}

impl IndexBuffer {

	pub fn bind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
		}

		return self;
	}

	pub fn unbind(&self) -> &Self {

		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
		}

		return self;

	}

	pub fn data(&self, data: &Vec<GLuint>) -> &Self {

		unsafe {

			self.bind();

			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(data.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
				mem::transmute(data.as_ptr()),
				gl::STATIC_DRAW
			);

			self.unbind();

		}

		return self;

	}

}

pub struct Mesh {

	buffers: Vec<Buffer>,
	index_buffer: IndexBuffer,

}

fn make_buffer() -> Buffer {

	unsafe {

		let mut id: GLuint = 0;

		gl::GenBuffers(1, &mut id);

		return Buffer {
			id: id,
		};

	}

}

fn make_index_buffer() -> IndexBuffer {

	unsafe {

		let mut id: GLuint = 0;

		gl::GenBuffers(1, &mut id);

		return IndexBuffer {
			id: id,
		};

	}

}

impl Mesh {

	pub fn make_buf(&mut self, data: &Vec<GLfloat>) -> &Buffer {

		unsafe {

			let buf = make_buffer();

			buf.data(&data);
			self.buffers.push(buf);

			return &self.buffers[self.buffers.len() - 1];

		}

	}

	pub fn make_index_buf(&mut self, data: &Vec<GLuint>) -> &IndexBuffer {

		unsafe {

			let buf = make_index_buffer();

			buf.data(&data);
			self.index_buffer = buf;

			return &self.index_buffer;

		}

	}

	pub fn draw(&self) {

		unsafe {
			self.index_buffer.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		}

	}

}

pub fn make_mesh() -> Mesh {

	return Mesh {
		buffers: vec![],
		index_buffer: IndexBuffer{
			id: 0
		},
	};

}

pub struct Texture {

	id: GLuint,
	pub width: u32,
	pub height: u32,

}

impl Texture {

	pub fn bind(&self) -> &Self {

		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}

		return self;

	}

	pub fn unbind(&self) -> &Self {

		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}

		return self;

	}

}

pub fn make_texture(pixels: &[u8], width: u32, height: u32) -> Texture {

	unsafe {

		let mut id: GLuint = 0;

		gl::GenTextures(1, &mut id);
		gl::BindTexture(gl::TEXTURE_2D, id);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
		gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
		gl::GenerateMipmap(gl::TEXTURE_2D);

		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGBA8 as GLint,
			width as GLint,
			height as GLint,
			0,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			pixels.as_ptr() as *const GLvoid
		);

		gl::BindTexture(gl::TEXTURE_2D, 0);

		return Texture {
			id: id,
			width: width,
			height: height,
		};

	}

}

pub struct Program {
	id: GLuint,
}

impl Program {

	pub fn attr(&self, index: GLuint, name: &str) -> &Self {

		unsafe {
			gl::BindAttribLocation(self.id, index, CString::new(name).unwrap().as_ptr());
		}

		return self;

	}

	pub fn bind(&self) -> &Self {

		unsafe {
			gl::UseProgram(self.id);
		}

		return self;

	}

	pub fn unbind(&self) -> &Self {

		unsafe {
			gl::UseProgram(0);
		}

		return self;

	}

	pub fn link(&self) -> &Self {

		unsafe {
			gl::LinkProgram(self.id);
		}

		return self;

	}

	pub fn uniform_vec4(&self, name: &str, value: [f32; 4]) -> &Self {

		unsafe {
			gl::Uniform4f(
				gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()),
				value[0],
				value[1],
				value[2],
				value[3],
			);
		}

		return self;

	}

	pub fn uniform_mat4(&self, name: &str, value: [[f32; 4]; 4]) -> &Self {

		unsafe {
			gl::UniformMatrix4fv(
				gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()),
				1,
				gl::FALSE,
				&value[0][0]
			);
		}

		return self;

	}

}

pub fn make_program(vs_src: String, fs_src: String) -> Program {

	unsafe {

		let vs: GLuint = compile_shader(gl::VERTEX_SHADER, vs_src);
		let fs: GLuint = compile_shader(gl::FRAGMENT_SHADER, fs_src);
		let id: GLuint = gl::CreateProgram();

		gl::AttachShader(id, vs);
		gl::AttachShader(id, fs);

		return Program {
			id: id
		};

	}

}

fn compile_shader(shader_type: GLenum, src: String) -> GLuint {

	unsafe {

		let id: GLuint = gl::CreateShader(shader_type);
		let src_cstr = CString::new(src).unwrap();

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
			panic!("{}", String::from_utf8(log).unwrap());

		}

		return id;

	}

}

pub fn init() {

	unsafe {

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		gl::ClearColor(0.0, 0.0, 0.0, 1.0);
		clear();

	}

}

pub fn clear() {
	unsafe {
		gl::Clear(gl::COLOR_BUFFER_BIT);
	}
}

