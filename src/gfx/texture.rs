// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextureConf {
	pub filter: FilterMode,
	pub wrap: WrapMode,
}

impl Default for TextureConf {
	fn default() -> Self {
		return Self {
			filter: FilterMode::Nearest,
			wrap: WrapMode::ClampToBorder,
		};
	}
}

/// 2D Texture
#[derive(Clone)]
pub struct Texture {
	gl_tex: Rc<TextureHandle>,
	gl: Rc<glow::Context>,
	width: i32,
	height: i32,
}

impl Texture {

	/// create a new empty texture with default conf
	pub fn new(ctx: &impl GLCtx, w: i32, h: i32) -> Result<Self> {
		return Self::new_with_conf(ctx, w, h, TextureConf::default());
	}

	/// create a new empty texture
	pub fn new_with_conf(ctx: &impl GLCtx, w: i32, h: i32, conf: TextureConf) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let gl_tex = TextureHandle::new(&gl)?;

			gl.bind_texture(glow::TEXTURE_2D, Some(gl_tex.id()));

			gl.tex_image_2d(
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

			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				conf.filter.as_glow(),
			);

			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				conf.filter.as_glow(),
			);

			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				conf.wrap.as_glow(),
			);

			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				conf.wrap.as_glow(),
			);

			gl.bind_texture(glow::TEXTURE_2D, None);

			return Ok(Self {
				gl_tex: Rc::new(gl_tex),
				gl: gl,
				width: w,
				height: h,
			});

		}

	}

	pub(super) fn new_depth_stencil(ctx: &impl GLCtx, w: i32, h: i32) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let gl_tex = TextureHandle::new(&gl)?;

			gl.bind_texture(glow::TEXTURE_2D, Some(gl_tex.id()));

			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::DEPTH24_STENCIL8 as i32,
				w,
				h,
				0,
				glow::DEPTH_STENCIL,
				glow::UNSIGNED_INT_24_8,
				None,
			);

			gl.bind_texture(glow::TEXTURE_2D, None);

			return Ok(Self {
				gl_tex: Rc::new(gl_tex),
				gl: gl,
				width: w,
				height: h,
			});

		}

	}

	pub fn from_raw_with_conf(ctx: &impl GLCtx, width: i32, height: i32, data: &[u8], conf: TextureConf) -> Result<Self> {
		let tex = Self::new_with_conf(ctx, width, height, conf)?;
		tex.data(data);
		return Ok(tex);
	}

	/// create a texture from raw pixels
	pub fn from_raw(ctx: &impl GLCtx, width: i32, height: i32, data: &[u8]) -> Result<Self> {
		return Self::from_raw_with_conf(ctx, width, height, data, TextureConf::default());
	}

	pub fn from_img_with_conf(ctx: &impl GLCtx, img: img::Image, conf: TextureConf) -> Result<Self> {
		return Self::from_raw_with_conf(ctx, img.width(), img.height(), &img.into_raw(), conf);
	}

	/// create a texture from an [`Image`](../img/struct.Image.html)
	pub fn from_img(ctx: &impl GLCtx, img: img::Image) -> Result<Self> {
		return Self::from_img_with_conf(ctx, img, TextureConf::default());
	}

	pub fn from_bytes_with_conf(ctx: &impl GLCtx, data: &[u8], conf: TextureConf) -> Result<Self> {
		return Self::from_img_with_conf(ctx, img::Image::from_bytes(data)?, conf);
	}

	/// create a texture from bytes read from an image file
	pub fn from_bytes(ctx: &impl GLCtx, data: &[u8]) -> Result<Self> {
		return Self::from_bytes_with_conf(ctx, data, TextureConf::default());
	}

	pub(super) fn bind(&self) {
		unsafe {
			self.gl.bind_texture(glow::TEXTURE_2D, Some(self.gl_tex.id()));
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
		let mut pixels = vec![0; size];

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

	pub(super) fn id(&self) -> glow::Texture {
		return self.gl_tex.id();
	}

}

impl PartialEq for Texture {
	fn eq(&self, other: &Self) -> bool {
		return self.gl_tex == other.gl_tex;
	}
}

