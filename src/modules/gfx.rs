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
use crate::utils::gl as ggl;

const MAX_DRAWS: usize = 65536;
const MAX_STATE_STACK: usize = 64;

// context
ctx!(GFX: GfxCtx);

struct GfxCtx {

	ibuf: ggl::IndexBuffer,
	vbuf: ggl::VertexBuffer,
	program: ggl::Program,
	empty_tex: Texture,
	projection: Mat4,
	state: State,
	state_stack: Vec<State>,
	default_font: Font,
	current_tex: Option<Texture>,
	vertex_queue: Vec<f32>,

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

	let indices: Vec<GLuint> = vec![0, 1, 3, 1, 2, 3]
		.iter()
		.cycle()
		.take(MAX_DRAWS * 6)
		.enumerate()
		.map(|(i, vertex)| vertex + i as u32 / 6 * 4)
		.collect();

	let vbuf = ggl::VertexBuffer::new(MAX_DRAWS * 4, 8, ggl::BufferUsage::Dynamic);

	vbuf
		.attr(0, 2, 0)
		.attr(1, 2, 2)
		.attr(2, 4, 4);

	let ibuf = ggl::IndexBuffer::new(MAX_DRAWS * 6, ggl::BufferUsage::Static);

	ibuf
		.data(&indices, 0);

	let program = ggl::Program::new(
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

		vbuf: vbuf,
		ibuf: ibuf,
		program: program,
		empty_tex: Texture::from_raw(&[255, 255, 255, 255], 1, 1),
		projection: projection,
		state_stack: Vec::with_capacity(64),
		state: State::default(),
		default_font: default_font,
		current_tex: None,
		vertex_queue: Vec::with_capacity(MAX_DRAWS * 4),

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

pub(crate) fn flush() {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();

	if let Some(tex) = gfx.current_tex {

		tex.bind();
		gfx.program.uniform_mat4("projection", gfx.projection.as_arr());
		gfx.vbuf.data(&gfx.vertex_queue, 0);
		ggl::draw(&gfx.vbuf, &gfx.ibuf, &gfx.program, gfx.vertex_queue.len() / 4 * 6);
		tex.unbind();
		gfx_mut.vertex_queue.clear();
		gfx_mut.current_tex = None;

	}

}

/// draw a texture with visible quad area
pub fn draw(tex: &Texture, quad: Rect) {

	let gfx = ctx_get();
	let gfx_mut = ctx_get_mut();
	let queue = &mut gfx_mut.vertex_queue;

	if let Some(current_tex) = gfx.current_tex {
		if current_tex != *tex {
			flush();
			gfx_mut.current_tex = Some(*tex);
		}
	} else {
		gfx_mut.current_tex = Some(*tex);
	}

	let mut push_vertex = |pos: Vec2, uv: Vec2, color: Color| {

		queue.push(pos.x);
		queue.push(pos.y);
		queue.push(uv.x);
		queue.push(uv.y);
		queue.push(color.r);
		queue.push(color.g);
		queue.push(color.b);
		queue.push(color.a);

	};

	let t = gfx.state.transform.scale(vec2!(tex.width as f32 * quad.w, tex.height as f32 * quad.h));
	let color = gfx.state.tint;

	push_vertex(t.forward(vec2!(0, 1)), vec2!(quad.x, quad.y + quad.h), color);
	push_vertex(t.forward(vec2!(1, 1)), vec2!(quad.x + quad.w, quad.y + quad.h), color);
	push_vertex(t.forward(vec2!(1, 0)), vec2!(quad.x + quad.w, quad.y), color);
	push_vertex(t.forward(vec2!(0, 0)), vec2!(quad.x, quad.y), color);

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

			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
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

