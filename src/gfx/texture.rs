// wengwengweng

use glow::HasContext;

use crate::*;
use gfx::*;

struct TextureHandle {
	gl: std::rc::Rc<glow::Context>,
	id: TextureID,
}

impl Drop for TextureHandle {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_texture(self.id);
		}
	}
}

/// 2D Texture
#[derive(Clone)]
pub struct Texture {
	handle: Rc<TextureHandle>,
	gl: Rc<glow::Context>,
	id: TextureID,
	width: i32,
	height: i32,
}

impl Texture {

	/// create a new empty texture with width & height
	pub fn new(ctx: &impl GLCtx, w: i32, h: i32) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let id = gl.create_texture()?;

			let handle = TextureHandle {
				id: id,
				gl: gl.clone(),
			};

			let tex = Self {
				handle: Rc::new(handle),
				gl: gl,
				id: id,
				width: w,
				height: h,
			};

			tex.bind();

			tex.gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				w,
				h,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				None,
			);

			tex.unbind();

			tex.set_filter(FilterMode::Nearest);
			tex.set_wrap(WrapMode::ClampToEdge);

			return Ok(tex);

		}

	}

	/// create a texture from raw pixels
	pub fn from_raw(ctx: &impl GLCtx, width: i32, height: i32, data: &[u8]) -> Result<Self> {

		let tex = Self::new(ctx, width, height)?;
		tex.data(data);
		return Ok(tex);

	}

	/// create a texture from an [`Image`](../img/struct.Image.html)
	pub fn from_img(ctx: &impl GLCtx, img: img::Image) -> Result<Self> {
		return Self::from_raw(ctx, img.width(), img.height(), &img.into_raw());
	}

	/// create a texture from bytes read from an image file
	pub fn from_bytes(ctx: &impl GLCtx, data: &[u8]) -> Result<Self> {
		return Self::from_img(ctx, img::Image::from_bytes(data)?);
	}

	/// set min/max filter mode
	pub fn set_filter(&self, f: FilterMode) {

		unsafe {

			self.bind();

			self.gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				f.to_glow(),
			);

			self.gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				f.to_glow(),
			);

			self.unbind();

		}

	}

	/// set wrap mode
	pub fn set_wrap(&self, w: WrapMode) {

		unsafe {

			self.bind();

			self.gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				w.to_glow(),
			);

			self.gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				w.to_glow(),
			);

			self.unbind();

		}

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.gl.bind_texture(glow::TEXTURE_2D, None);
		}
	}

	pub(super) fn sub_data(&self, x: i32, y: i32, w: i32, h: i32, data: &[u8]) {

		unsafe {

			self.bind();

			self.gl.tex_sub_image_2d(
				glow::TEXTURE_2D,
				0,
				x as i32,
				y as i32,
				w as i32,
				h as i32,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				glow::PixelUnpackData::Slice(data),
			);

			self.unbind();

		}

	}

	pub(super) fn data(&self, data: &[u8]) {
		self.sub_data(0, 0, self.width, self.height, data);
	}

	/// get texture width
	pub fn width(&self) -> i32 {
		return self.width;
	}

	/// get texture width
	pub fn height(&self) -> i32 {
		return self.height;
	}

	/// capture content to an [`Image`](../img/struct.Image.html)
	pub fn capture(&self) -> Result<img::Image> {

		let size = (self.width * self.height * 4) as usize;
		let mut pixels = vec![0.0 as u8; size];

		self.bind();

		unsafe {

			self.gl.get_tex_image(
				glow::TEXTURE_2D,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				glow::PixelPackData::Slice(&mut pixels),
			);

		}

		self.unbind();

		return img::Image::from_raw(self.width, self.height, pixels);

	}

	pub(super) fn id(&self) -> TextureID {
		return self.id;
	}

}

impl PartialEq for Texture {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

