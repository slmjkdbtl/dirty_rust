// wengwengweng

//! Image decoding & encoding

use std::path::Path;

#[cfg(feature = "fs")]
use crate::fs;
#[cfg(not(feature = "fs"))]
use std::fs;

use crate::Result;

pub use image::ImageFormat as Format;

pub struct Image {
	handle: image::RgbaImage,
}

impl Image {

	pub fn new(w: u32, h: u32) -> Result<Self> {
		return Ok(Self {
			handle: image::ImageBuffer::new(w, h),
		});
	}

	pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_bytes(&fs::read(path)?);
	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {

		let img = image::load_from_memory(data)?
			.to_rgba();

		return Ok(Image {
			handle: img,
		});

	}

	pub fn from_pixels(w: u32, h: u32, pixels: &[u8]) -> Self {
		unimplemented!();
	}

	pub fn width(&self) -> u32 {
		return self.handle.width();
	}

	pub fn height(&self) -> u32 {
		return self.handle.height();
	}

	pub fn write(&self, path: impl AsRef<Path>) -> Result<()> {
		return Ok(self.handle.save(path)?);
	}

	pub fn into_raw(self) -> Vec<u8> {
		return self.handle.into_raw();
	}

}

