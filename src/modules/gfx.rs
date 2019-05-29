// wengwengweng

use std::sync::Arc;

use crate::math::*;
use crate::img::Image;
use crate::*;

pub struct Ctx {
	transform: Mat4,
	stack: Vec<Mat4>,
	// ...
}

impl Ctx {

	pub fn new() -> Self {
		return Self {
			transform: Mat4::identity(),
			stack: Vec::with_capacity(64),
		};
	}

}

pub trait G2d {

	fn matrix(&self) -> Mat4;
	fn set_matrix(&mut self, m: Mat4);
	fn push(&mut self);
	fn pop(&mut self);

	fn translate(&mut self, pos: Vec2) {
		self.set_matrix(self.matrix().translate(vec3!(pos.x, pos.y, 0.0)));
	}

	fn rotate(&mut self, angle: f32) {
		self.set_matrix(self.matrix().rotate(angle, Dir::Z));
	}

	fn scale(&mut self, s: Vec2) {
		self.set_matrix(self.matrix().scale(vec3!(s.x, s.y, 0.0)));
	}

}

pub trait G3d {
	// ...
}

impl G2d for Ctx {

	fn matrix(&self) -> Mat4 {
		return self.transform;
	}

	fn set_matrix(&mut self, m: Mat4) {
		self.transform = m;
	}

	fn push(&mut self) {
		self.stack.push(self.transform);
	}

	fn pop(&mut self) {
		self.stack.pop().expect("cannot pop anymore");
	}

}

#[derive(Clone)]
pub struct Texture {
	handle: Arc<ggl::Texture>,
}

impl Texture {

	pub fn from_image(img: Image) -> Self {

		let handle = ggl::Texture::new(img.width(), img.height());

		handle.data(&img.into_raw());

		return Self {
			handle: Arc::new(handle),
		};

	}

	pub fn from_file(fname: &str) -> Result<Self> {
		return Ok(Self::from_image(Image::from_file(fname)?));
	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {
		return Ok(Self::from_image(Image::from_bytes(data)?));
	}

	pub fn from_pixels(w: u32, h: u32, pixels: &[u8]) -> Self {
		return Self::from_image(Image::from_pixels(w, h, pixels));
	}

}

/// offscreen framebuffer
#[derive(Clone)]
pub struct Canvas {

	handle: Arc<ggl::Framebuffer>,
	tex: Texture,
	width: u32,
	height: u32,

}

impl Canvas {

	/// create new canvas
	pub fn new(width: u32, height: u32) -> Self {

		let handle = ggl::Framebuffer::new();
		let pixels = vec![0.0 as u8; (width * height * 4) as usize];
		let tex = Texture::from_pixels(width, height, &pixels);

		handle.attach(&*tex.handle);

		return Self {
			handle: Arc::new(handle),
			tex: tex,
			width: width,
			height: height,
		}

	}

}

