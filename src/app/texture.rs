// wengwengweng

use std::path::Path;
use std::rc::Rc;

use crate::*;
use super::*;

#[cfg(feature = "img")]
use crate::img::Image;

#[derive(Clone, PartialEq)]
pub struct Texture {
	pub(super) handle: Rc<gl::Texture>,
}

impl Texture {

	pub(super) fn from_handle(handle: gl::Texture) -> Self {
		return Self {
			handle: Rc::new(handle),
		};
	}

	pub fn new(ctx: &Ctx, w: i32, h: i32) -> Result<Self> {
		return Ok(Self::from_handle(gl::Texture::new(&ctx.gl, w, h)?));
	}

	#[cfg(feature = "img")]
	pub fn from_img(ctx: &Ctx, img: Image) -> Result<Self> {

		let w = img.width();
		let h = img.height();

		return Self::from_pixels(ctx, w, h, &img.into_raw());

	}

	#[cfg(feature = "img")]
	pub fn from_bytes(ctx: &Ctx, data: &[u8]) -> Result<Self> {
		return Self::from_img(ctx, Image::from_bytes(data)?);
	}

	pub fn from_pixels(ctx: &Ctx, w: i32, h: i32, pixels: &[u8]) -> Result<Self> {

		let handle = gl::Texture::from(&ctx.gl, w, h, &pixels)?;
		handle.filter(ctx.conf.texture_filter);
		return Ok(Self::from_handle(handle));

	}

	pub fn width(&self) -> i32 {
		return self.handle.width();
	}

	pub fn height(&self) -> i32 {
		return self.handle.height();
	}

	pub fn get_pixels(&self) -> Vec<u8> {
		return self.handle.get_data(self.width(), self.height());
	}

	pub(super) fn data(&self, data: &[u8]) {
		self.handle.data(data);
	}

	pub(super) fn sub_data(&self, x: i32, y: i32, w: i32, h: i32, data: &[u8]) {
		self.handle.sub_data(x, y, w, h, data);
	}

	#[cfg(feature = "img")]
	pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {

		image::save_buffer(
			path,
			&self.get_pixels(),
			self.width() as u32,
			self.height() as u32,
			image::ColorType::RGBA(8),
		)?;

		return Ok(());

	}

}

