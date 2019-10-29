// wengwengweng

use std::rc::Rc;

use glow::HasContext;

use super::*;
use crate::Result;

pub trait Texture {
	fn id(&self) -> TextureID;
	fn r#type(&self) -> TextureType;
}

#[derive(Clone, Debug)]
pub struct Texture2D {
	ctx: Rc<GLCtx>,
	id: TextureID,
	width: i32,
	height: i32,
}

impl Texture for Texture2D {
	fn id(&self) -> TextureID {
		return self.id;
	}
	fn r#type(&self) -> TextureType {
		return TextureType::Tex2D;
	}
}

impl Texture2D {

	pub fn new(device: &Device, width: i32, height: i32) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_texture()?;

			let tex = Self {
				ctx: ctx,
				id: id,
				width: width,
				height: height,
			};

			tex.bind();

			tex.ctx.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				width,
				height,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				None,
			);

			tex.unbind();

			tex.set_filter(FilterMode::Nearest);
			tex.set_wrap(WrapMode::Repeat);

			return Ok(tex);

		}

	}

	pub fn from(device: &Device, width: i32, height: i32, data: &[u8]) -> Result<Self> {

		let tex = Self::new(device, width, height)?;
		tex.data(data);
		return Ok(tex);

	}

	pub(super) fn id(&self) -> TextureID {
		return self.id;
	}

	pub fn set_filter(&self, f: FilterMode) {

		unsafe {

			self.bind();

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				f.into(),
			);

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				f.into(),
			);

			self.unbind();

		}

	}

	pub fn set_wrap(&self, w: WrapMode) {

		unsafe {

			self.bind();

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				w.into(),
			);

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				w.into(),
			);

			self.unbind();

		}

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_2D, Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_2D, None);
		}
	}

	pub fn width(&self) -> i32 {
		return self.width as i32;
	}

	pub fn height(&self) -> i32 {
		return self.height as i32;
	}

	pub fn sub_data(&self, x: i32, y: i32, w: i32, h: i32, data: &[u8]) {

		unsafe {

			self.bind();

			self.ctx.tex_sub_image_2d_u8_slice(
				glow::TEXTURE_2D,
				0,
				x as i32,
				y as i32,
				w as i32,
				h as i32,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(data),
			);

			self.unbind();

		}

	}

	pub fn data(&self, data: &[u8]) {
		self.sub_data(0, 0, self.width, self.height, data);
	}

	pub fn get_data(&self, width: i32, height: i32) -> Vec<u8> {

		let size = (width * height * 4) as usize;
		let pixels = vec![0.0 as u8; size];

		self.bind();

		unsafe {

			self.ctx.get_tex_image_u8_slice(
				glow::TEXTURE_2D,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(&pixels),
			);

		}

		self.unbind();

		return pixels;

	}

	pub fn drop(&self) {
		unsafe {
			self.ctx.delete_texture(self.id);
		}
	}

}

impl PartialEq for Texture2D {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

#[derive(Clone, Debug)]
pub struct CubemapTexture {
	ctx: Rc<GLCtx>,
	id: TextureID,
	width: i32,
	height: i32,
}

impl Texture for CubemapTexture {
	fn id(&self) -> TextureID {
		return self.id;
	}
	fn r#type(&self) -> TextureType {
		return TextureType::Cubemap;
	}
}

impl CubemapTexture {

	pub fn from(
		device: &Device,
		width: i32,
		height: i32,
		right: &[u8],
		left: &[u8],
		up: &[u8],
		down: &[u8],
		back: &[u8],
		front: &[u8],
	) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_texture()?;

			let tex = Self {
				ctx: ctx,
				id: id,
				width: width,
				height: height,
			};

			tex.bind();

			let bind = |side: CubemapSide, data: &[u8]| {
				tex.ctx.tex_image_2d(
					side.into(),
					0,
					glow::RGBA as i32,
					width,
					height,
					0,
					glow::RGBA,
					glow::UNSIGNED_BYTE,
					Some(data),
				);
			};

			use CubemapSide::*;

			bind(Left, left);
			bind(Right, right);
			bind(Up, up);
			bind(Down, down);
			bind(Back, back);
			bind(Front, front);

			tex.unbind();

			tex.set_filter(FilterMode::Nearest);
			tex.set_wrap(WrapMode::Repeat);

			return Ok(tex);

		}

	}

	pub(super) fn id(&self) -> TextureID {
		return self.id;
	}

	pub fn set_filter(&self, f: FilterMode) {

		unsafe {

			self.bind();

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_CUBE_MAP,
				glow::TEXTURE_MIN_FILTER,
				f.into(),
			);

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_CUBE_MAP,
				glow::TEXTURE_MAG_FILTER,
				f.into(),
			);

			self.unbind();

		}

	}

	pub fn set_wrap(&self, w: WrapMode) {

		unsafe {

			self.bind();

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_CUBE_MAP,
				glow::TEXTURE_WRAP_S,
				w.into(),
			);

			self.ctx.tex_parameter_i32(
				glow::TEXTURE_CUBE_MAP,
				glow::TEXTURE_WRAP_T,
				w.into(),
			);

			self.unbind();

		}

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_CUBE_MAP, Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.ctx.bind_texture(glow::TEXTURE_CUBE_MAP, None);
		}
	}

	pub fn width(&self) -> i32 {
		return self.width as i32;
	}

	pub fn height(&self) -> i32 {
		return self.height as i32;
	}

	pub fn sub_data(
		&self,
		side: CubemapSide,
		x: i32,
		y: i32,
		w: i32,
		h: i32,
		data: &[u8]
	) {

		unsafe {

			self.bind();

			self.ctx.tex_sub_image_2d_u8_slice(
				side.into(),
				0,
				x as i32,
				y as i32,
				w as i32,
				h as i32,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(data),
			);

			self.unbind();

		}

	}

	pub fn data(&self, side: CubemapSide, data: &[u8]) {
		self.sub_data(side, 0, 0, self.width, self.height, data);
	}

	pub fn get_data(&self, width: i32, height: i32) -> Vec<u8> {

		let size = (width * height * 4) as usize;
		let pixels = vec![0.0 as u8; size];

		self.bind();

		unsafe {

			self.ctx.get_tex_image_u8_slice(
				glow::TEXTURE_CUBE_MAP,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(&pixels),
			);

		}

		self.unbind();

		return pixels;

	}

	pub fn drop(&self) {
		unsafe {
			self.ctx.delete_texture(self.id);
		}
	}

}

impl PartialEq for CubemapTexture {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

