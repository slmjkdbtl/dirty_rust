// wengwengweng

use std::rc::Rc;

use glow::HasContext;

use super::*;
use crate::Result;

#[derive(Clone, Debug)]
pub struct Framebuffer {
	ctx: Rc<GLCtx>,
	id: FramebufferID,
	tex: Texture2D,
}

impl Framebuffer {

	pub fn new(device: &Device, w: i32, h: i32) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_framebuffer()?;

			let pixels = vec![0.0 as u8; (w * h * 4) as usize];
			let tex = Texture2D::from(&device, w, h, &pixels)?;

			let fbuf = Self {
				ctx,
				id,
				tex,
			};

			fbuf.bind();

			fbuf.ctx.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(fbuf.tex.id()),
				0,
			);

			if fbuf.ctx.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
				return Err("failed to create framebuffer".to_string());
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

	pub fn tex(&self) -> &Texture2D {
		return &self.tex;
	}

	pub(super) fn id(&self) -> FramebufferID {
		return self.id;
	}

	pub(super) fn bind(&self) {
		unsafe {
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.ctx.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
	}

	pub fn free(self) {
		unsafe {
			self.tex.free();
			self.ctx.delete_framebuffer(self.id);
		}
	}

}

impl PartialEq for Framebuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

