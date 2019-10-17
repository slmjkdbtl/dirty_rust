// wengwengweng

use std::rc::Rc;
use std::path::Path;

use crate::*;
use super::*;
use super::gfx::*;

#[derive(Clone, PartialEq)]
pub struct Canvas {
	pub(super) handle: Rc<gl::Framebuffer>,
	pub(super) tex: Texture,
}

impl Canvas {

	pub fn new(ctx: &Ctx, width: i32, height: i32) -> Result<Self> {

		let dpi = ctx.dpi();
		let tw = (width as f64 * dpi) as i32;
		let th = (height as f64 * dpi) as i32;
		let handle = gl::Framebuffer::new(&ctx.gl, tw, th)?;
		let tex = Texture::from_handle(handle.tex().clone());

		return Ok(Self {
			handle: Rc::new(handle),
			tex: tex,
		});

	}

	pub fn width(&self) -> i32 {
		return self.tex.width();
	}

	pub fn height(&self) -> i32 {
		return self.tex.height();
	}

	pub fn tex(&self) -> &Texture {
		return &self.tex;
	}

	#[cfg(feature = "img")]
	pub fn capture(&self, path: impl AsRef<Path>) -> Result<()> {
		return self.tex.save(path);
	}

}

