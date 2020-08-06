// wengwengweng

use glow::HasContext;

use crate::*;
use gfx::*;

struct CanvasHandle {
	gl: std::rc::Rc<glow::Context>,
	fbo: FramebufferID,
	rbo: RenderbufferID,
}

impl Drop for CanvasHandle {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_renderbuffer(self.rbo);
			self.gl.delete_framebuffer(self.fbo);
		}
	}
}

/// Off-screen Rendering Canvas
#[derive(Clone)]
pub struct Canvas {
	handle: Rc<CanvasHandle>,
	gl: Rc<glow::Context>,
	id: FramebufferID,
	rbo: RenderbufferID,
	tex: Texture,
	width: i32,
	height: i32,
}

impl Canvas {

	/// create a new canvas with width & height
	pub fn new(ctx: &Gfx, w: i32, h: i32) -> Result<Self> {

		let dpi = ctx.dpi();
		let tw = (w as f32 * dpi) as i32;
		let th = (h as f32 * dpi) as i32;

		unsafe {

			let gl = ctx.gl().clone();
			let id = gl.create_framebuffer()?;

			let pixels = vec![0.0 as u8; (tw * th * 4) as usize];
			let tex = Texture::from_raw(ctx, tw, th, &pixels)?;

			let rbo = gl.create_renderbuffer()?;

			gl.bind_renderbuffer(glow::RENDERBUFFER, Some(rbo));

			gl.renderbuffer_storage(
				glow::RENDERBUFFER,
				glow::DEPTH_STENCIL,
				tw as i32,
				th as i32,
			);

			gl.bind_renderbuffer(glow::RENDERBUFFER, None);

			let handle = CanvasHandle {
				gl: gl.clone(),
				fbo: id,
				rbo: rbo,
			};

			let fbuf = Self {
				handle: Rc::new(handle),
				gl: gl,
				id: id,
				tex: tex,
				rbo: rbo,
				width: w,
				height: h,
			};

			fbuf.bind();

			fbuf.gl.clear(Surface::Color.to_glow());
			fbuf.gl.clear(Surface::Depth.to_glow());
			fbuf.gl.clear(Surface::Stencil.to_glow());

			fbuf.gl.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D,
				Some(fbuf.tex.id()),
				0,
			);

			fbuf.gl.framebuffer_renderbuffer(
				glow::FRAMEBUFFER,
				glow::DEPTH_STENCIL_ATTACHMENT,
				glow::RENDERBUFFER,
				Some(rbo),
			);

			if fbuf.gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
				return Err(format!("failed to create framebuffer"));
			}

			fbuf.unbind();

			return Ok(fbuf);

		}

	}

	pub(super) fn id(&self) -> FramebufferID {
		return self.id;
	}

	pub(super) fn bind(&self) {
		unsafe {
			self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
		}
	}

	/// get canvas width
	pub fn width(&self) -> i32 {
		return self.width;
	}

	/// get canvas height
	pub fn height(&self) -> i32 {
		return self.height;
	}

	/// get canvas texture
	pub fn tex(&self) -> &Texture {
		return &self.tex;
	}

	/// capture content to an [`Image`](../img/struct.Image.html)
	pub fn capture(&self) -> Result<img::Image> {
		return self.tex.capture();
	}

}

impl PartialEq for Canvas {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

