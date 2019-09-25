// wengwengweng

use std::rc::Rc;

use glow::Context;

use super::*;
use crate::Result;

#[derive(Clone, Debug)]
pub struct Framebuffer {

	ctx: Rc<GLCtx>,
	pub(super) id: FramebufferID,

}

impl Framebuffer {

	pub fn new(device: &Device, tex: &Texture) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_framebuffer()?;

			let rbo = ctx.create_renderbuffer()?;

			ctx.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo));

			ctx.renderbuffer_storage(
				glow::RENDERBUFFER,
				glow::DEPTH24_STENCIL8,
				tex.width as i32,
				tex.height as i32
			);

			ctx.bind_renderbuffer(glow::RENDERBUFFER, None);

			let fbuf = Self {
				ctx: ctx,
				id: id,
			};

			fbuf.bind();

			fbuf.ctx.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(tex.id),
				0,
			);

			fbuf.ctx.framebuffer_renderbuffer(
				glow::FRAMEBUFFER,
				glow::DEPTH_STENCIL_ATTACHMENT,
				glow::RENDERBUFFER,
				Some(rbo),
			);

			if fbuf.ctx.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
				return Err(Error::OpenGL("failed to create framebuffer".to_owned()));
			}

			device.clear(Surface::Depth);
			device.clear(Surface::Stencil);

			fbuf.unbind();

			return Ok(fbuf);

		}
	}

	pub fn with<R>(&self, f: impl FnOnce() -> R) -> R {

		self.bind();
		let r = f();
		self.unbind();

		return r;

	}

	fn bind(&self) {
		unsafe {
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
		}
	}

	fn unbind(&self) {
		unsafe {
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
	}

}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_framebuffer(self.id);
		}
	}
}

impl PartialEq for Framebuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

