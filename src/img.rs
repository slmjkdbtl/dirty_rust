// wengwengweng

//! Image decoding & encoding

use std::path::Path;

use crate::math::Color;
use crate::Result;
use crate::Error;

pub use image::ImageFormat as Format;

pub struct Image {
	handle: image::RgbaImage,
}

impl Image {

	pub fn new(w: i32, h: i32) -> Result<Self> {
		return Ok(Self {
			handle: image::ImageBuffer::new(w as u32, h as u32),
		});
	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {

		let img = image::load_from_memory(data)?
			.to_rgba();

		return Ok(Image {
			handle: img,
		});

	}

	pub fn width(&self) -> i32 {
		return self.handle.width() as i32;
	}

	pub fn height(&self) -> i32 {
		return self.handle.height() as i32;
	}

	pub fn write(&self, path: impl AsRef<Path>) -> Result<()> {
		return Ok(self.handle.save(path)?);
	}

	pub fn get(&self, x: i32, y: i32) -> Option<Color> {

		if x < 0 || x > self.width() - 1 || y < 0 || y > self.height() - 1 {
			return None;
		}

		return Some(self.handle.get_pixel(x as u32, y as u32).into());

	}

	pub fn set(&mut self, x: i32, y: i32, c: Color) -> Result<()> {

		if x < 0 || x > self.width() - 1 || y < 0 || y > self.height() - 1 {
			return Err(Error::Image(format!("pixel out of bound: ({}, {})", x, y)));
		}

		return Ok(self.handle.put_pixel(x as u32, y as u32, c.into()));

	}

	pub fn resize(&mut self) -> Result<()> {
		todo!();
	}

	pub fn into_raw(self) -> Vec<u8> {
		return self.handle.into_raw();
	}

}

impl From<&image::Rgba<u8>> for Color {
	fn from(c: &image::Rgba<u8>) -> Color {
		return Color::new(
			c.0[0] as f32 / 255.0,
			c.0[1] as f32 / 255.0,
			c.0[2] as f32 / 255.0,
			c.0[3] as f32 / 255.0,
		);
	}
}

impl From<Color> for image::Rgba<u8> {
	fn from(c: Color) -> image::Rgba<u8> {
		return image::Rgba(c.to_rgba());
	}
}

