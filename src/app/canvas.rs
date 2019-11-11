// wengwengweng

use std::rc::Rc;
use std::path::Path;

use crate::*;
use super::*;
use super::gfx::*;

pub struct CanvasBuilder {
	origin: Option<Origin>,
	size: Option<(i32, i32)>,
}

impl CanvasBuilder {
	pub fn origin(mut self, o: Origin) -> Self {
		self.origin = Some(o);
		return self;
	}
	pub fn size(mut self, w: i32, h: i32) -> Self {
		self.size = Some((w, h));
		return self;
	}
	pub fn build(self, ctx: &Ctx) -> Result<Canvas> {

		let (w, h) = self.size.unwrap_or((ctx.gwidth(), ctx.gheight()));
		let dpi = ctx.dpi();
		let tw = (w as f32 * dpi) as i32;
		let th = (h as f32 * dpi) as i32;
		let fbuf = gl::Framebuffer::new(&ctx.gl, tw, th)?;
		let tex = Texture::from_gl_tex(fbuf.tex().clone());

		return Ok(Canvas {
			gl_fbuf: Rc::new(fbuf),
			tex: tex,
			origin: self.origin.unwrap_or(ctx.conf.origin),
		});

	}
}

/// framebuffer for off-screen rendering
#[derive(Clone, PartialEq)]
pub struct Canvas {
	gl_fbuf: Rc<gl::Framebuffer>,
	tex: Texture,
	origin: Origin,
}

impl Canvas {

	pub fn builder() -> CanvasBuilder {
		return CanvasBuilder {
			origin: None,
			size: None,
		};
	}

	/// create a new canvas from width and height
	pub fn new(ctx: &Ctx, width: i32, height: i32) -> Result<Self> {

		return Self::builder()
			.size(width, height)
			.build(ctx);

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

