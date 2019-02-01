// wengwengweng

//! Rendering

use std::rc::Rc;

use crate::*;
use crate::math::*;
use crate::backends::gl;

include!("../res/resources.rs");

// context
ctx!(GFX: GfxCtx);

struct GfxCtx {
	current_canvas: Option<Canvas>,
}

pub(super) fn init() {

	ctx_init(GfxCtx {
		current_canvas: None,
	});

	g3d::init();
	g2d::init();
	gl::set_blend(gl::BlendFac::SourceAlpha, gl::BlendFac::OneMinusSourceAlpha);
	gl::set_depth(gl::DepthFunc::LessOrEqual);
	gl::clear_color(color!(0, 0, 0, 1));
	clear();
	window::swap();

}

/// check if gfx is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

/// render a canvas
pub fn render(c: &Canvas) {
	g2d::draw(&c.tex, rect!(0, 0, 1, 1));
}

/// set active canvas
pub fn drawon(c: &Canvas) {

	let gfx = ctx_get_mut();

	assert!(gfx.current_canvas.is_none(), "cannot draw on canvas while another canvas is active");

	g2d::flush();
	g2d::flip_projection();
	gl::set_framebuffer(&*c.handle);
	gfx.current_canvas = Some(c.clone());

}

/// stop active canvas
pub fn stop_drawon(c: &Canvas) {

	let gfx = ctx_get_mut();

	if let Some(current) = &gfx.current_canvas {

		assert!(current == c, "this is not the active canvas");

		g2d::flush();
		g2d::unflip_projection();
		gl::unset_framebuffer(&*c.handle);
		gfx.current_canvas = None;

	} else {
		panic!("no canvas active");
	}

}

/// clear view
pub fn clear() {
	gl::clear(true, true, false);
}

/// save a canvas into a png file
pub fn capture(canvas: &Canvas, fname: &str) {

	let tex = &canvas.tex;
	let buffer = tex.handle.get_data();

	image::save_buffer(
		fname,
		&buffer,
		tex.width(),
		tex.height(),
		image::ColorType::RGBA(8),
	).expect("failed to save png");

}

pub(super) fn begin() {
	clear();
}

pub(super) fn end() {

	let gfx = ctx_get();

	g2d::flush();
	g2d::reset();
	g2d::clear_stack();
	g3d::reset();
	g3d::clear_stack();

	if gfx.current_canvas.is_some() {
		panic!("unfinished canvas");
	}

}

/// texture
#[derive(PartialEq, Clone)]
pub struct Texture {
	pub(super) handle: Rc<gl::Texture>,
}

impl Texture {

	/// create an empty texture with width and height
	pub fn new(width: u32, height: u32) -> Self {
		return Self {
			handle: Rc::new(gl::Texture::new(width, height)),
		};
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

		let tex = Self::new(width, height);

		tex.handle.data(pixels);

		return tex;

	}

	/// create texture from a file
	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

	pub fn from_color(c: Color, width: u32, height: u32) -> Self {
		return Self::from_raw(&c.to_rgba(), width, height);
	}

	/// get texture width
	pub fn width(&self) -> u32 {
		return self.handle.width;
	}

	/// get texture height
	pub fn height(&self) -> u32 {
		return self.handle.height;
	}

}

/// offscreen framebuffer
#[derive(PartialEq, Clone)]
pub struct Canvas {

	handle: Rc<gl::Framebuffer>,
	tex: Texture,
	width: u32,
	height: u32,

}

impl Canvas {

	/// create new canvas
	pub fn new(width: u32, height: u32) -> Self {

		let handle = gl::Framebuffer::new();
		let pixels = vec![0.0 as u8; (width * height * 4) as usize];
		let tex = Texture::from_raw(&pixels, width, height);

		handle.attach(&*tex.handle);

		return Self {
			handle: Rc::new(handle),
			tex: tex,
			width: width,
			height: height,
		}

	}

}

