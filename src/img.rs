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

	pub fn new(w: i32, h: i32) -> Result<Self> {
		return Self::from_raw(w, h, vec![0; w as usize * h as usize * 4]);
	}

	pub fn from_raw(w: i32, h: i32, data: Vec<u8>) -> Result<Self> {

		if data.len() != w as usize * h as usize * 4 {
			return Err("incorrect image size".to_string());
		}

		if w <= 0 || h <= 0 {
			return Err("image size must be > 0".to_string())
		}

		return Ok(Self {
			data,
			width: w,
			height: h,
		});

	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {

		let img = image::load_from_memory(data)
			.map_err(|_| "failed to parse img".to_string())?
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

	// TODO
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

	// TODO
	pub fn set(&mut self, x: i32, y: i32, c: Color) {

		let i = (y * self.width * 4 + x * 4) as usize;

		if let Some(r) = self.data.get_mut(i) {
			*r = (c.r * 255.0) as u8;
		}

		if let Some(g) = self.data.get_mut(i + 1) {
			*g = (c.g * 255.0) as u8;
		}

		if let Some(b) = self.data.get_mut(i + 2) {
			*b = (c.b * 255.0) as u8;
		}

		if let Some(a) = self.data.get_mut(i + 3) {
			*a = (c.a * 255.0) as u8;
		}

	}

	fn into_image(self) -> Result<image::RgbaImage> {
		let img: image::RgbaImage = image::ImageBuffer::from_raw(self.width as u32, self.height as u32, self.data)
			.ok_or_else(|| "failed to create image".to_string())?;
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

	// TODO
	pub fn flip_v(&self) -> Self {

		let mut img = self.clone();

		for y in 0..self.height {
			for x in 0..self.width {
				if let Some(p) = self.get(x, y) {
					img.set(x, self.height - y, p);
				}
			}
		}

		return img;

	}

}

