// wengwengweng

use crate::math::*;
use crate::img::Image;
use crate::*;

pub struct Ctx {
	// ...
}

pub struct State {
	// ...
}

impl Ctx {
	pub fn translate(p: Vec2) {
		let a = vec2!();
	}
}

pub struct Texture {
	// ...
}

impl Texture {

	pub fn from_image(img: Image) -> Self {
		return Self {};
	}

	pub fn from_file(fname: &str) -> Result<Self> {
		return Ok(Self::from_image(Image::from_file(fname)?));
	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {
		return Ok(Self::from_image(Image::from_bytes(data)?));
	}

	pub fn from_raw(w: u32, h: u32, pixels: &[u8]) -> Result<Self> {
		return Ok(Self::from_image(Image::from_raw(w, h, pixels)?));
	}

}

