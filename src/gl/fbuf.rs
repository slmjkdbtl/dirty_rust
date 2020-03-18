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
	rbo: RenderbufferID,
}

impl Framebuffer {

	pub fn new(device: &Device, w: i32, h: i32) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_framebuffer()?;

			let pixels = vec![0.0 as u8; (w * h * 4) as usize];
			let tex = Texture2D::from(&device, w, h, &pixels)?;

			let rbo = ctx.create_renderbuffer()?;

			ctx.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo));

			ctx.renderbuffer_storage(
				glow::RENDERBUFFER,
				glow::DEPTH24_STENCIL8,
				w as i32,
				h as i32,
			);

			ctx.bind_renderbuffer(glow::RENDERBUFFER, None);

			let fbuf = Self {
				ctx: ctx,
				id: id,
				tex: tex,
				rbo: rbo,
			};

			fbuf.bind();

			fbuf.ctx.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(fbuf.tex.id()),
				0,
			);

			fbuf.ctx.framebuffer_renderbuffer(
				glow::FRAMEBUFFER,
				glow::DEPTH_STENCIL_ATTACHMENT,
				glow::RENDERBUFFER,
				Some(rbo),
			);

			if fbuf.ctx.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
				return Err(format!("failed to create framebuffer"));
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
			self.ctx.delete_framebuffer(self.id);
			self.ctx.delete_renderbuffer(self.rbo);
		}
	}

}

impl PartialEq for Framebuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

