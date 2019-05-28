// wengwengweng

//! Image decoding & encoding

use std::path::Path;

use crate::fs;
use crate::math::Color;
use crate::Result;
use crate::Error;

pub struct Image {
	handle: image::RgbaImage,
}

impl Image {

	pub fn from_raw(w: u32, h: u32, buf: &[u8]) -> Result<Self> {
		return Ok(Self {
			handle: image::ImageBuffer::from_raw(w, h, buf.to_vec()).ok_or(Error::Image)?,
		});
	}

	pub fn from_file(fname: &str) -> Result<Self> {
		return Self::from_bytes(&fs::read(fname)?);
	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {

		let img = image::load_from_memory(data)?
			.to_rgba();

		return Ok(Image {
			handle: img,
		});

	}

	pub fn width(&self) -> u32 {
		return self.handle.width();
	}

	pub fn height(&self) -> u32 {
		return self.handle.height();
	}

	pub fn write(&self, fname: impl AsRef<Path>) -> Result<()> {
		return Ok(self.handle.save(fname)?);
	}

}

