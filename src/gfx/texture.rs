// wengwengweng

use crate::*;
use gfx::*;
use img::*;

/// 2D Texture
#[derive(Clone, PartialEq)]
pub struct Texture {
	gl_tex: gl::Texture2D,
}

impl Texture {

	pub(crate) fn from_gl_tex(gl_tex: gl::Texture2D) -> Self {
		return Self {
			gl_tex,
		};
	}

	/// create an empty texture
	pub fn new(ctx: &impl HasGLDevice, w: i32, h: i32) -> Result<Self> {
		return Ok(Self::from_gl_tex(gl::Texture2D::new(&ctx.device(), w, h)?));
	}

	/// create texture from an [Image](img::Image)
	pub fn from_img(ctx: &impl HasGLDevice, img: Image) -> Result<Self> {

		let w = img.width();
		let h = img.height();

		return Self::from_pixels(ctx, w, h, &img.into_raw());

	}

	/// create texture from bytes read from an image file
	pub fn from_bytes(ctx: &impl HasGLDevice, data: &[u8]) -> Result<Self> {
		return Self::from_img(ctx, Image::from_bytes(data)?);
	}

	/// create texture from raw pixels
	pub fn from_pixels(ctx: &impl HasGLDevice, w: i32, h: i32, pixels: &[u8]) -> Result<Self> {

		let gl_tex = gl::Texture2D::from(&ctx.device(), w, h, &pixels)?;
		return Ok(Self::from_gl_tex(gl_tex));

	}

	/// texture width
	pub fn width(&self) -> i32 {
		return self.gl_tex.width();
	}

	/// texture height
	pub fn height(&self) -> i32 {
		return self.gl_tex.height();
	}

	/// get texture pixel data
	pub fn get_pixels(&self) -> Vec<u8> {
		return self.gl_tex.get_data(self.width(), self.height());
	}

	/// set texture pixel data
	pub fn data(&self, data: &[u8]) {
		self.gl_tex.data(data);
	}

	/// set texture pixel data of an area
	pub fn sub_data(&self, x: i32, y: i32, w: i32, h: i32, data: &[u8]) {
		self.gl_tex.sub_data(x, y, w, h, data);
	}

	/// free texture memory
	pub fn free(self) {
		self.gl_tex.free();
	}

	pub(crate) fn gl_tex(&self) -> &gl::Texture2D {
		return &self.gl_tex;
	}

}

