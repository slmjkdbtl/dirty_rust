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
}

impl Canvas {

	/// create a new canvas from width and height
	pub fn new(ctx: &Ctx, width: i32, height: i32) -> Result<Self> {

		let dpi = ctx.dpi();
		let tw = (width as f32 * dpi) as i32;
		let th = (height as f32 * dpi) as i32;
		let fbuf = gl::Framebuffer::new(&ctx.gl, tw, th)?;
		let tex = Texture::from_gl_tex(fbuf.tex().clone());

		return Ok(Self {
			gl_fbuf: Rc::new(fbuf),
			tex: tex,
		});

	}

	/// get canvas width
	pub fn width(&self) -> i32 {
		return self.tex.width();
	}

	/// get canvas height
	pub fn height(&self) -> i32 {
		return self.tex.height();
	}

	/// get underlying texture for the canvas
	pub fn tex(&self) -> &Texture {
		return &self.tex;
	}

	/// capture and save to an image
	#[cfg(feature = "img")]
	pub fn capture(&self, path: impl AsRef<Path>) -> Result<()> {
		return self.tex.save(path);
	}

	pub(super) fn gl_fbuf(&self) -> &gl::Framebuffer {
		return &self.gl_fbuf;
	}

}

