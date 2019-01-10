// wengwengweng

//! Rendering

use std::ptr;
use std::mem;
use std::ffi::CString;
use std::ffi::c_void;
use std::collections::HashMap;

use gl::types::*;

use crate::*;
use crate::math::mat::Mat4;

const MAX_SPRITES: usize = 2048;
const MAX_STATE_STACK: usize = 64;

// context
ctx!(GFX: GfxCtx);

struct GfxCtx {

	mesh: Mesh,
	program: Program,
	empty_tex: Texture,
	projection: Mat4,
	state: State,
	state_stack: Vec<State>,
	default_font: Font,
	current_tex: Option<Texture>,
	vertex_queue: Vec<f32>,
	draw_calls: usize,

}

#[derive(Clone, Copy)]
struct State {
	transform: Mat4,
	tint: Color,
	line_width: u8,
}

impl Default for State {
	fn default() -> Self {
		return Self {
			transform: Mat4::identity(),
			tint: color!(),
			line_width: 1,
		}
	}
}

pub(crate) fn init() {

	unsafe {

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		gl::ClearColor(0.0, 0.0, 0.0, 1.0);

	}

	clear();
	window::swap();

	let indices: Vec<GLuint> = vec![
		0, 1, 3,
		1, 2, 3,
	]
		.iter()
		.cycle()
		.take(MAX_SPRITES * 6)
		.enumerate()
		.map(|(i, vertex)| vertex + i as u32 / 6 * 4)
		.collect();

	let mut mesh = Mesh::new();

	mesh.make_buf(MAX_SPRITES * 4).attr(0, 2, 8, 0).attr(1, 2, 8, 2).attr(2, 4, 8, 4);;
	mesh.make_index_buf(&indices);

	let program = Program::new(
		include_str!("../shaders/quad.vert"),
		include_str!("../shaders/quad.frag"),
	);

	program
		.attr(0, "pos")
		.attr(1, "uv")
		.attr(2, "color")
		.link();

	let default_font = Font::new(
		Texture::from_bytes(include_bytes!("../misc/CP437.png")),
		32,
		8,
		r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##,
	);

	let (width, height) = window::size();
	let projection = Mat4::ortho(0.0, (width as f32), (height as f32), 0.0, -1.0, 1.0);

	ctx_init(GfxCtx {

		mesh: mesh,
		program: program,
		empty_tex: Texture::from_raw(&[255, 255, 255, 255], 1, 1),
		projection: projection,
		state_stack: Vec::with_capacity(64),
		state: State::default(),
		default_font: default_font,
		current_tex: None,
		vertex_queue: Vec::with_capacity(1024),
		draw_calls: 0,

	});

}

/// check if gfx is initiated
pub fn enabled() -> bool {
	return ctx_is_ok();
}

/// reset global transforms and style states
pub fn reset() {

	let gfx_mut = ctx_get_mut();

	gfx_mut.state_stack.clear();
	gfx_mut.state = State::default();

}

fn push_vertex(pos: Vec2, uv: Vec2, color: Color) {

	let ctx_mut = ctx_get_mut();
	let queue = &mut ctx_mut.vertex_queue;

	queue.push(pos.x);
	queue.push(pos.y);
	queue.push(uv.x);
	queue.push(uv.y);
	queue.push(color.r);
	queue.push(color.g);
	queue.push(color.b);
	queue.push(color.a);

}

pub(crate) fn flush() {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();

	if let Some(tex) = gfx.current_tex {

		tex.bind();
		gfx.program
			.uniform_mat4("projection", gfx.projection.as_arr())
			.bind();
		gfx.mesh.buffers[0].update(&gfx.vertex_queue, 0);
		gfx.mesh.draw();
		tex.unbind();
		gfx_mut.vertex_queue.clear();
		gfx_mut.draw_calls += 1;
		gfx_mut.current_tex = None;

	}

}

/// get draw calls
pub fn draw_calls() -> usize {
	return ctx_get().draw_calls;
}

pub(crate) fn update() {
	ctx_get_mut().draw_calls = 0;
}

/// draw a texture with visible quad area
pub fn draw(tex: &Texture, quad: Rect) {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();

	if let Some(current_tex) = gfx.current_tex {
		if current_tex != *tex {
			flush();
			gfx_mut.current_tex = Some(*tex);
		}
	} else {
		gfx_mut.current_tex = Some(*tex);
	}

	push();
	scale(vec2!(tex.width as f32 * quad.w, tex.height as f32 * quad.h));

	let trans = gfx.state.transform;
	let color = gfx.state.tint;

	push_vertex(trans.forward(vec2!(0, 1)), vec2!(quad.x, quad.y + quad.h), color);
	push_vertex(trans.forward(vec2!(1, 1)), vec2!(quad.x + quad.w, quad.y + quad.h), color);
	push_vertex(trans.forward(vec2!(1, 0)), vec2!(quad.x + quad.w, quad.y), color);
	push_vertex(trans.forward(vec2!(0, 0)), vec2!(quad.x, quad.y), color);
	pop();

}

/// draw canvas
pub fn render(canvas: &Canvas) {
	draw(&canvas.tex, rect!(0, 0, 1, 1));
}

/// draw text
pub fn text(s: &str) {

	let gfx = ctx_get();
	let font = &gfx.default_font;

	for (i, ch) in s.chars().enumerate() {

		push();
		translate(vec2!(i as f32 * font.grid_size.x * font.tex.width as f32, 0));

		if ch != ' ' {
			draw(&font.tex, *font.map.get(&ch).unwrap_or_else(|| panic!("does not have char '{}'", ch)));
		}

		pop();

	}

}

/// draw rectangle with size
pub fn rect(size: Vec2) {

	let gfx = ctx_get();

	push();
	scale(size);
	draw(&gfx.empty_tex, rect!(0, 0, 1, 1));
	pop();

}

/// draw line
pub fn line(p1: Vec2, p2: Vec2) {

	let gfx = ctx_get();
	let len = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
	let rot = (p2.y - p1.y).atan2(p2.x - p1.x);

	push();
	translate(p1);
	rotate(rot);
	rect(vec2!(len, gfx.state.line_width));
	pop();

}

/// draw polygon with vertices
pub fn poly(pts: &[Vec2]) {

	for (i, p) in pts.iter().enumerate() {

		if (i == pts.len() - 1) {
			line(*p, pts[0]);
		} else {
			line(*p, pts[i + 1]);
		}

	}

}

/// set global tint
pub fn color(tint: Color) {
	ctx_get_mut().state.tint = tint;
}

/// set line width
pub fn line_width(line_width: u8) {
	ctx_get_mut().state.line_width = line_width;
}

/// push state
pub fn push() {

	let gfx = ctx_get_mut();
	let stack = &mut gfx.state_stack;

	if (stack.len() < 64) {
		stack.push(gfx.state);
	} else {
		panic!("cannot push anymore");
	}

}

/// pop state
pub fn pop() {

	let mut gfx = ctx_get_mut();
	let stack = &mut gfx.state_stack;

	gfx.state = stack.pop().expect("cannot pop anymore");

}

/// global translate
pub fn translate(pos: Vec2) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.translate(pos);

}

/// global rotate
pub fn rotate(rot: f32) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.rotate(rot);

}

/// global scale
pub fn scale(s: Vec2) {

	let state = &mut ctx_get_mut().state;

	state.transform = state.transform.scale(s);


}

/// warp a 2d point through current transformed matrix
pub fn warp(pt: Vec2) -> Vec2 {

	let gfx = ctx_get();
	let trans = gfx.state.transform;

	return trans.forward(pt);

}

/// inverse warp a 2d point through current transformed matrix
pub fn inverse_warp(pt: Vec2) -> Vec2 {

	let gfx = ctx_get();
	let trans = gfx.state.transform;

	return trans.inverse().forward(pt);

}

/// clear view
pub fn clear() {

	unsafe {
		gl::Clear(gl::COLOR_BUFFER_BIT);
	}

}

/// start drawing on a canvas
pub fn draw_on(canvas: &Canvas) {
	canvas.bind();
}

/// stop drawing on a canvas
pub fn stop_draw_on(canvas: &Canvas) {
	canvas.unbind();
}

/// texture
#[derive(Clone, Copy, PartialEq)]
pub struct Texture {

	id: GLuint,
	/// width
	pub width: u32,
	/// height
	pub height: u32,

}

/// texture scaling filter
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Filter {
	/// nearest
	Nearest,
	/// linear
	Linear,
}

impl Texture {

	/// create an empty texture with width and height
	pub fn new(width: u32, height: u32) -> Self {

		let mut id: GLuint = 0;

		unsafe {
			gl::GenTextures(1, &mut id);
		}

		return Self {

			id: id,
			width: width,
			height: height,

		}

	}

	/// create texture with raw data
	pub fn from_bytes(data: &[u8]) -> Self {

		let img = image::load_from_memory(data)
			.expect("failed to load image")
			.to_rgba();

		let width = img.width();
		let height = img.height();
		let pixels = img.into_raw();

		return Self::from_raw(&pixels, width, height);

	}

	/// create texture from pixel data, width and height
	pub fn from_raw(pixels: &[u8], width: u32, height: u32) -> Self {

		let mut tex = Self::new(width, height);

		tex.data(pixels);

		return tex;

	}

	/// create texture from a file
	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

	fn data(&mut self, pixels: &[u8]) -> &Self {

		self.bind();

		unsafe {

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
			gl::GenerateMipmap(gl::TEXTURE_2D);

			gl::TexImage2D(

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

}

/// frame buffer
pub struct Canvas {

	tex: Texture,
	id: GLuint,

}

impl Canvas {

	/// create a frame buffer from width and height
	pub fn new(width: u32, height: u32) -> Self {

		let mut id: GLuint = 0;
		let mut rbo: GLuint = 0;
		let tex = Texture::from_raw(&[], width, height);

		unsafe {

			gl::GenFramebuffers(1, &mut id);
			gl::BindFramebuffer(gl::FRAMEBUFFER, id);
			gl::DrawBuffer(gl::COLOR_ATTACHMENT0);

			gl::FramebufferTexture2D(gl::DRAW_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex.id, 0);

			gl::GenRenderbuffers(1, &mut rbo);
			gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
			gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT16, width as GLint, height as GLint);
			gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

			gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, rbo);

			if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
				panic!("canvas init failed");
			}

			clear();
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

		}

		return Self {

			id: id,
			tex: tex,

		}

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

/// bitmap font
pub struct Font {

	tex: Texture,
	map: HashMap<char, Rect>,
	grid_size: Vec2,

}

impl Font {

	/// creat a bitmap font from a texture, and grid of characters
	pub fn new(tex: Texture, cols: usize, rows: usize, chars: &str) -> Self {

		let mut map = HashMap::new();
		let grid_size = vec2!(1.0 / cols as f32, 1.0 / rows as f32);

		assert_eq!(tex.width % cols as u32, 0, "font size not right");
		assert_eq!(tex.height % rows as u32, 0, "font size not right");

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, rect!(

				(i % cols) as f32 * grid_size.x,
				(i / cols) as f32 * grid_size.y,
				grid_size.x,
				grid_size.y

			));

		}

		return Self {

			tex: tex,
			map: map,
			grid_size: grid_size,

		}

	}

}

// struct DrawState {

// 	verts: Vec<GLfloat>,
// 	uvs: Vec<GLfloat>,
// 	colors: Vec<GLfloat>,
// 	indices: Vec<GLuint>,
// 	count: u32,

// }

struct Buffer {
	id: GLuint,
}

impl Buffer {

	fn new(count: usize) -> Self {

		let mut id: GLuint = 0;

		unsafe {

			gl::GenBuffers(1, &mut id);
			gl::BindBuffer(gl::ARRAY_BUFFER, id);

			gl::BufferData(
				gl::ARRAY_BUFFER,
				(count * mem::size_of::<GLfloat>()) as GLsizeiptr,
				ptr::null() as *const c_void,
				gl::DYNAMIC_DRAW
			);

			gl::BindBuffer(gl::ARRAY_BUFFER, 0);

		}

		return Self {
			id: id,
		};

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

	fn update(&self, data: &[GLfloat], offset: usize) -> &Self {

		unsafe {

			self.bind();

            gl::BufferSubData(
				gl::ARRAY_BUFFER,
				(offset * mem::size_of::<GLfloat>()) as GLsizeiptr,
				(data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
				data.as_ptr() as *const c_void,
            );

			self.unbind();

		}

		return self;

	}

	fn attr(&self, attr_index: GLuint, buf_size: GLint, stride: usize, offset: usize) -> &Self {

		unsafe {

			self.bind();

			gl::VertexAttribPointer(
				attr_index,
				buf_size,
				gl::FLOAT,
				gl::FALSE,
				(stride * mem::size_of::<f32>()) as i32,
				(offset * mem::size_of::<f32>()) as *const c_void
			);

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

	fn data(&mut self, data: &[GLuint]) -> &Self {

		unsafe {

			self.size = data.len() as GLint;
			self.bind();

			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(data.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
				data.as_ptr() as *const c_void,
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
			index_buffer: IndexBuffer {
				id: 0,
				size: 0,
			},
		};

	}

	fn make_buf(&mut self, count: usize) -> &Buffer {

		let buf = Buffer::new(count);

		self.buffers.push(buf);

		return &self.buffers[self.buffers.len() - 1];

	}

	fn make_index_buf(&mut self, data: &[GLuint]) -> &IndexBuffer {

		let mut buf = IndexBuffer::new();

		buf.data(&data);
		self.index_buffer = buf;

		return &self.index_buffer;

	}

	fn draw(&self) {

		unsafe {
			self.buffers[0].bind();
			self.index_buffer.bind();
			gl::DrawElements(gl::TRIANGLES, self.index_buffer.size, gl::UNSIGNED_INT, ptr::null());
		}

	}

}

struct Program {
	id: GLuint,
}

impl Program {

	fn new(vs_src: &str, fs_src: &str) -> Self {

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
			gl::BindAttribLocation(self.id, index, cstr(name).as_ptr());
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

	fn uniform_color(&self, name: &str, c: Color) -> &Self {
		return self.uniform_vec4(name, vec4!(c.r, c.g, c.b, c.a));
	}

	fn uniform_rect(&self, name: &str, r: Rect) -> &Self {
		return self.uniform_vec4(name, vec4!(r.x, r.y, r.w, r.h));
	}

	fn uniform_vec4(&self, name: &str, v: Vec4) -> &Self {

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

	fn uniform_mat4(&self, name: &str, value: [[f32; 4]; 4]) -> &Self {

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

fn cstr(name: &str) -> CString {
	return CString::new(name).expect("failed to parse cstring");
}

fn compile_shader(shader_type: GLenum, src: &str) -> GLuint {

	unsafe {

		let id: GLuint = gl::CreateShader(shader_type);
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

