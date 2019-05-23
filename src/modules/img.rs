// wengwengweng

//! Image decoding & encoding

use std::path::Path;

use serde::Serialize;
use serde::Deserialize;

use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct Image {
	width: u32,
	height: u32,
	pixels: Vec<u8>,
}

impl Image {

	pub fn width(&self) -> u32 {
		return self.width;
	}

	pub fn height(&self) -> u32 {
		return self.height;
	}

	pub fn pixels(&self) -> &Vec<u8> {
		return &self.pixels;
	}

	pub fn write_png(&self, fname: impl AsRef<Path>) -> Result<()> {

		image::save_buffer(
			fname,
			&self.pixels,
			self.width,
			self.height,
			image::ColorType::RGBA(8)
		)?;

		return Ok(());

	}

}

pub fn decode_png(data: &[u8]) -> Result<Image> {

	let img = image::load_from_memory(data)?
		.to_rgba();

	return Ok(Image {
		width: img.width(),
		height: img.height(),
		pixels: img.into_raw(),
	});

}

