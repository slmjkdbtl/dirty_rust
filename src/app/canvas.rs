// wengwengweng

use std::rc::Rc;
use std::path::Path;

use crate::*;
use super::*;
use super::gfx::*;

/// framebuffer for off-screen rendering
#[derive(Clone, PartialEq)]
pub struct Canvas {
	gl_fbuf: Rc<gl::Framebuffer>,
	tex: Texture,
	width: i32,
	height: i32,
}

impl Canvas {

	pub fn new(ctx: &Ctx, w: i32, h: i32) -> Result<Self> {

		let dpi = ctx.dpi();
		let tw = (w as f32 * dpi) as i32;
		let th = (h as f32 * dpi) as i32;
		let fbuf = gl::Framebuffer::new(&ctx.gl, tw, th)?;
		let tex = Texture::from_gl_tex(fbuf.tex().clone());

		return Ok(Self {
			gl_fbuf: Rc::new(fbuf),
			tex: tex,
			width: w,
			height: h,
		});

	}

	// TODO: give original size
	/// get canvas width
	pub fn width(&self) -> i32 {
		return self.width;
	}

	/// get canvas height
	pub fn height(&self) -> i32 {
		return self.height;
	}

	/// get underlying texture for the canvas
	pub fn tex(&self) -> &Texture {
		return &self.tex;
	}

	/// capture and save to an image
	#[cfg(feature = "img")]
	pub fn capture(&self, path: impl AsRef<Path>) -> Result<()> {

		let path = path.as_ref();

		let image = image::ImageBuffer::from_raw(
			self.tex.width() as u32,
			self.tex.height() as u32,
			self.tex.get_pixels()
		).ok_or(format!("failed to write image to {}", path.display()))?;

		let image = image::DynamicImage::ImageRgba8(image).flipv();

		image
			.save(path)
			.map_err(|_| format!("failed to write image to {}", path.display()))?;

		return Ok(());

	}

	pub(super) fn gl_fbuf(&self) -> &gl::Framebuffer {
		return &self.gl_fbuf;
	}

}

