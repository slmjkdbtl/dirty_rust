// wengwengweng

use std::rc::Rc;

use glow::HasContext;

use super::*;
use crate::Result;

#[derive(Clone, Debug)]
pub struct Texture {
	ctx: Rc<GLCtx>,
	id: TextureID,
}

impl Texture {

	pub fn empty(device: &Device) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_texture()?;

			let tex = Self {
				ctx: ctx,
				id: id,
			};

			tex.bind();

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				glow::REPEAT as i32
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				glow::REPEAT as i32
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				FilterMode::Nearest.into(),
			);

			tex.ctx.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				FilterMode::Nearest.into(),
			);

			tex.unbind();

			return Ok(tex);

		}

	}

	pub fn new(device: &Device, width: i32, height: i32) -> Result<Self> {

		unsafe {

			let tex = Self::empty(device)?;

			tex.bind();

			tex.ctx.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				width as i32,
				height as i32,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				None,
			);

			tex.unbind();

			return Ok(tex);

		}

	}

	pub fn from(device: &Device, width: i32, height: i32, data: &[u8]) -> Result<Self> {

		let tex = Self::new(device, width, height)?;
		tex.data(0, 0, width, height, data);
		return Ok(tex);

	}

	pub(super) fn id(&self) -> TextureID {
		return self.id;
	}

	pub fn filter(&self, f: FilterMode) {

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

	pub fn data(&self, x: i32, y: i32, width: i32, height: i32, data: &[u8]) {

		unsafe {

			self.bind();

			self.ctx.tex_sub_image_2d_u8_slice(
				glow::TEXTURE_2D,
				0,
				x,
				y,
				width as i32,
				height as i32,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(data),
			);

			self.unbind();

		}

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

impl PartialEq for Texture {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

