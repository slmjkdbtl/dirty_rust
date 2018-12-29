// wengwengweng

use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::mem;

use crate::ctx;
use crate::app;
use crate::math::*;

ctx!(GFX: GfxCtx);

pub fn init() {

	unsafe {

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		gl::ClearColor(0.0, 0.0, 0.0, 1.0);

	}

	clear();

	init_ctx(GfxCtx {
		renderer_2d: Renderer2D::new(),
	});

}

pub fn update() {

	let g = get_ctx_mut();

	g.renderer_2d.g_trans_stack.clear();

}

struct GfxCtx {
	renderer_2d: Renderer2D,
}

pub fn draw(tex: &Texture, pos: Vec2, r: f32, s: Vec2, quad: Rect, tint: Color) {

	let g = get_ctx();
	let renderer = &g.renderer_2d;
	let (width, height) = app::size();
	let proj = Mat4::ortho(0.0, (width as f32), (height as f32), 0.0, -1.0, 1.0);
	let quad = quad;

	push();
	translate(pos.x, pos.y);
	rotate(r);
	scale((tex.width as f32) * quad.w * s.x, (tex.height as f32) * quad.h * s.y);

	tex.bind();

	renderer.program
		.uniform_vec4("tint", tint.as_arr())
		.uniform_vec4("quad", quad.as_arr())
		.uniform_mat4("proj", proj.as_arr())
		.uniform_mat4("trans", renderer.g_trans.as_arr())
		.bind();

	pop();

	renderer.mesh.draw();
	tex.unbind();

}

pub fn rect(quad: Rect, r: f32, tint: Color) {

	let g = get_ctx();
	let renderer = &g.renderer_2d;

	draw(&renderer.empty_tex, Vec2::new(quad.x, quad.y), r, Vec2::new(quad.w, quad.h), Rect::new(0.0, 0.0, 1.0, 1.0), tint);

}

pub fn line(p1: Vec2, p2: Vec2, width: u8, tint: Color) {

	let cx = p1.x + (p2.x - p1.x) / 2.0;
	let cy = p1.y + (p2.y - p1.y) / 2.0;
	let dis = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
	let rot = (p2.y - p1.y).atan2(p2.x - p1.x);

	rect(Rect::new(cx, cy, dis, width as f32), rot, tint);

}

pub fn push() {

	let g = get_ctx_mut();
	let stack = &mut g.renderer_2d.g_trans_stack;

	if (stack.len() < 32) {
		stack.push(g.renderer_2d.g_trans);
	} else {
		panic!("cannot push anymore");
	}

}

pub fn pop() {

	let mut g = get_ctx_mut();
	let stack = &mut g.renderer_2d.g_trans_stack;

	match stack.pop() {
		Some(t) => {
			g.renderer_2d.g_trans = t;
		}
		None => {
			panic!("cannot pop anymore");
		}
	}

}

pub fn translate(x: f32, y: f32) {

	let g = get_ctx_mut();
	let r = &mut g.renderer_2d;

	r.g_trans = r.g_trans.translate(x, y);

}

pub fn rotate(rot: f32) {

	let g = get_ctx_mut();
	let r = &mut g.renderer_2d;

	r.g_trans = r.g_trans.rotate(rot);

}

pub fn scale(sx: f32, sy: f32) {

	let g = get_ctx_mut();
	let r = &mut g.renderer_2d;

	r.g_trans = r.g_trans.scale(sx, sy);

}

struct Renderer2D {

	mesh: Mesh,
	program: Program,
	empty_tex: Texture,
	g_trans: Mat4,
	g_trans_stack: Vec<Mat4>,

}

impl Renderer2D {

	fn new() -> Self {

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

		let mut mesh = Mesh::new();

		mesh.make_buf(&vertices).attr(0, 2);
		mesh.make_buf(&uv).attr(1, 2);
		mesh.make_index_buf(&indices);

		let program = Program::new(
			include_str!("quad.vert").to_owned(),
			include_str!("quad.frag").to_owned()
		);

		program
			.attr(0, "pos")
			.attr(1, "uv")
			.link();

		return Self {
			mesh: mesh,
			program: program,
			empty_tex: Texture::from_raw(&[255, 255, 255, 255], 1, 1),
			g_trans: Mat4::identity(),
			g_trans_stack: vec![],
		};

	}

}

struct Buffer {
	id: GLuint,
}

impl Buffer {

	fn new() -> Self {

		unsafe {

			let mut id: GLuint = 0;

			gl::GenBuffers(1, &mut id);

			return Self {
				id: id,
			};

		}

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

	fn data(&self, data: &Vec<GLfloat>) -> &Self {

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

	fn attr(&self, attr_index: GLuint, buf_size: GLint) -> &Self {

		unsafe {

			self.bind();
			gl::VertexAttribPointer(attr_index, buf_size, gl::FLOAT, gl::FALSE, 0, ptr::null());
			gl::EnableVertexAttribArray(attr_index);
			self.unbind();

		}

		return self;

	}

}

struct IndexBuffer {
	id: GLuint,
	size: GLint,
}

impl IndexBuffer {

	fn new() -> Self {

		unsafe {

			let mut id: GLuint = 0;

			gl::GenBuffers(1, &mut id);

			return Self {
				id: id,
				size: 0,
			};

		}

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

	fn data(&mut self, data: &Vec<GLuint>) -> &Self {

		unsafe {

			self.size = data.len() as GLint;
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

struct Mesh {

	buffers: Vec<Buffer>,
	index_buffer: IndexBuffer,

}

impl Mesh {

	fn new() -> Self {

		return Self {
			buffers: vec![],
			index_buffer: IndexBuffer{
				id: 0,
				size: 0,
			},
		};

	}

	fn make_buf(&mut self, data: &Vec<GLfloat>) -> &Buffer {

		let buf = Buffer::new();

		buf.data(&data);
		self.buffers.push(buf);

		return &self.buffers[self.buffers.len() - 1];

	}

	fn make_index_buf(&mut self, data: &Vec<GLuint>) -> &IndexBuffer {

		let mut buf = IndexBuffer::new();

		buf.data(&data);
		self.index_buffer = buf;

		return &self.index_buffer;

	}

	fn draw(&self) {

		unsafe {
			self.index_buffer.bind();
			gl::DrawElements(gl::TRIANGLES, self.index_buffer.size, gl::UNSIGNED_INT, ptr::null());
		}

	}

}

pub struct Texture {

	id: GLuint,
	pub width: u32,
	pub height: u32,

}

impl Texture {

	pub fn from_raw(pixels: &[u8], width: u32, height: u32) -> Self {

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

	pub fn from_bytes(data: &[u8]) -> Self {

		let img = image::load(std::io::Cursor::new(data), image::PNG)
			.unwrap()
			.to_rgba();

		let width = img.width();
		let height = img.height();
		let pixels = img.into_raw();

		return Texture::from_raw(&pixels, width, height);

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

}

struct Program {
	id: GLuint,
}

impl Program {

	fn new(vs_src: String, fs_src: String) -> Self {

		unsafe {

			let vs: GLuint = compile_shader(gl::VERTEX_SHADER, vs_src);
			let fs: GLuint = compile_shader(gl::FRAGMENT_SHADER, fs_src);
			let id: GLuint = gl::CreateProgram();

			gl::AttachShader(id, vs);
			gl::AttachShader(id, fs);

			return Self {
				id: id
			};

		}

	}

	fn attr(&self, index: GLuint, name: &str) -> &Self {

		unsafe {
			gl::BindAttribLocation(self.id, index, CString::new(name).unwrap().as_ptr());
		}

		return self;

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

	fn link(&self) -> &Self {

		unsafe {
			gl::LinkProgram(self.id);
		}

		return self;

	}

	fn uniform_vec4(&self, name: &str, value: [f32; 4]) -> &Self {

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

	fn uniform_mat4(&self, name: &str, value: [[f32; 4]; 4]) -> &Self {

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

pub fn clear() {

	unsafe {
		gl::Clear(gl::COLOR_BUFFER_BIT);
	}

}

