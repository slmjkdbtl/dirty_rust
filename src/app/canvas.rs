// wengwengweng

use std::rc::Rc;
use std::path::Path;

use crate::*;
use super::*;
use super::gfx::*;

#[derive(Clone, Copy, Debug)]
pub struct CanvasConf {
	size: (i32, i32),
	origin: Origin,
}

/// framebuffer for off-screen rendering
#[derive(Clone, PartialEq)]
pub struct Canvas {
	gl_fbuf: Rc<gl::Framebuffer>,
	tex: Texture,
	origin: Origin,
	size: (i32, i32),
}

impl Canvas {

	pub fn from_conf(ctx: &Ctx, conf: &CanvasConf) -> Result<Self> {

		let dpi = ctx.dpi();
		let (w, h) = conf.size;
		let tw = (w as f32 * dpi) as i32;
		let th = (h as f32 * dpi) as i32;
		let fbuf = gl::Framebuffer::new(&ctx.gl, tw, th)?;
		let tex = Texture::from_gl_tex(fbuf.tex().clone());

		return Ok(Self {
			gl_fbuf: Rc::new(fbuf),
			tex: tex,
			origin: conf.origin,
			size: conf.size,
		});

	}

	/// create a new canvas from width and height
	pub fn new(ctx: &Ctx, width: i32, height: i32) -> Result<Self> {

		return Self::from_conf(ctx, &CanvasConf {
			size: (width, height),
			origin: ctx.conf.origin,
		});

	}

	// TODO: give original size
	/// get canvas width
	pub fn width(&self) -> i32 {
		return self.size.0;
	}

	/// get canvas height
	pub fn height(&self) -> i32 {
		return self.size.1;
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

	/// get underlying texture for the canvas
	pub fn origin(&self) -> gfx::Origin {
		return self.origin;
	}

	pub(super) fn gl_fbuf(&self) -> &gl::Framebuffer {
		return &self.gl_fbuf;
	}

}

