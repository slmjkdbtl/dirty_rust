// wengwengweng

//! Image Buffer

use std::path::Path;

use crate::math::Color;
use crate::math::rgba;
use crate::Result;

use serde::Serialize;
use serde::Deserialize;
pub use image::imageops::FilterType;

#[derive(Clone, Serialize, Deserialize)]
pub struct Image {
	data: Vec<u8>,
	width: i32,
	height: i32,
}

impl Image {

	pub fn new(w: i32, h: i32) -> Self {
		return Self {
			data: vec![0; w as usize * h as usize * 4],
			width: w,
			height: h,
		};
	}

	pub fn from_raw(w: i32, h: i32, data: Vec<u8>) -> Result<Self> {

		if data.len() != w as usize * h as usize * 4 {
			return Err(format!("incorrect image size"));
		}

		if w <= 0 || h <= 0 {
			return Err(format!("image size must be > 0"));
		}

		return Ok(Self {
			data: data,
			width: w,
			height: h,
		});

	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {

		let img = image::load_from_memory(data)
			.map_err(|_| format!("failed to parse img"))?
			.to_rgba()
			;

		return Ok(Image {
			width: img.width() as i32,
			height: img.height() as i32,
			data: img.into_raw(),
		});

	}

	pub fn width(&self) -> i32 {
		return self.width;
	}

	pub fn height(&self) -> i32 {
		return self.height;
	}

	pub fn get(&self, x: i32, y: i32) -> Option<Color> {

		if x < 0 || y < 0 {
			return None;
		}

		let i = (y * self.width * 4 + x * 4) as usize;
		let r = *self.data.get(i)? as f32 / 255.0;
		let g = *self.data.get(i + 1)? as f32 / 255.0;
		let b = *self.data.get(i + 2)? as f32 / 255.0;
		let a = *self.data.get(i + 3)? as f32 / 255.0;

		return Some(rgba!(r, g, b, a));

	}

	pub fn set(&mut self, x: i32, y: i32, c: Color) -> Result<()> {

		let i = (y * self.width * 4 + x * 4) as usize;
		let (r, g, b, a) = c.as_u8();

		*self.data.get_mut(i).ok_or_else(|| format!("pixel out of bound"))? = r;
		*self.data.get_mut(i + 1).ok_or_else(|| format!("pixel out of bound"))? = g;
		*self.data.get_mut(i + 2).ok_or_else(|| format!("pixel out of bound"))? = b;
		*self.data.get_mut(i + 3).ok_or_else(|| format!("pixel out of bound"))? = a;

		return Ok(());

	}

	fn into_image(self) -> Result<image::RgbaImage> {
		let img: image::RgbaImage = image::ImageBuffer::from_raw(self.width as u32, self.height as u32, self.data)
			.ok_or_else(|| format!("failed to create image"))?;
		return Ok(img);
	}

	fn from_image(img: image::RgbaImage) -> Self {
		return Self {
			width: img.width() as i32,
			height: img.height() as i32,
			data: img.into_raw(),
		};
	}

	pub fn resize(self, w: i32, h: i32, filter: FilterType) -> Result<Self> {

		let img = self.into_image()?;
		let img = image::imageops::resize(&img, w as u32, h as u32, filter);

		return Ok(Self::from_image(img));

	}

	pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {

		let path = path.as_ref();

		image::save_buffer(
			path,
			&self.data,
			self.width as u32,
			self.height as u32,
			image::ColorType::Rgba8
		).map_err(|_| format!("failed to write image to {}", path.display()))?;

		return Ok(());

	}

	pub fn into_raw(self) -> Vec<u8> {
		return self.data;
	}

	pub fn flip_v(self) -> Self {

		let mut img = Self::new(self.width, self.height);

		for y in 0..self.height {
			for x in 0..self.width {
				if let Some(p) = self.get(x, y) {
					img.set(x, self.height - y, p).ok();
				}
			}
		}

		return img;

	}

}

