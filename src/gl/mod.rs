// wengwengweng

//! OpenGL Abstraction

export!(types);
export!(texture);
export!(pipeline);
export!(vbuf);
export!(ibuf);
export!(fbuf);
export!(attr);
#[cfg(feature = "gl3")]
export!(vao);
export!(uniform);
export!(stencil);
export!(mesh);
export!(shape);
export!(batch);

use std::mem;
use std::rc::Rc;
use std::marker::PhantomData;

use glow::HasContext;

use crate::Result;

pub(self) type GLCtx = glow::Context;

#[cfg(web)]
use webgl_stdweb::WebGL2RenderingContext;

pub(self) type BufferID = <GLCtx as HasContext>::Buffer;
pub(self) type ProgramID = <GLCtx as HasContext>::Program;
pub(self) type TextureID = <GLCtx as HasContext>::Texture;
pub(self) type FramebufferID = <GLCtx as HasContext>::Framebuffer;

#[cfg(feature = "gl3")]
pub(self) type VertexArrayID = <GLCtx as HasContext>::VertexArray;

pub struct Device {
	ctx: Rc<GLCtx>,
}

// TODO: web
// TODO: clean up this mess
impl Device {

	#[cfg(not(web))]
	pub fn from_loader<F: FnMut(&str) -> *const std::os::raw::c_void>(f: F) -> Self {
		return Self {
			ctx: Rc::new(GLCtx::from_loader_function(f)),
		};
	}

	#[cfg(web)]
	pub fn from_webgl2_ctx(c: WebGL2RenderingContext) -> Self {
		return Self {
			ctx: Rc::new(GLCtx::from_webgl2_context(c)),
		};
	}

	pub fn enable(&self, cap: Capability) {
		unsafe {
			self.ctx.enable(cap.into());
		}
	}

	pub fn disable(&self, cap: Capability) {
		unsafe {
			self.ctx.disable(cap.into());
		}
	}

	pub fn blend_func(&self, src: BlendFac, dest: BlendFac) {
		unsafe {
			self.ctx.blend_func(src.into(), dest.into());
		}
	}

	pub fn blend_func_sep(&self, src_rgb: BlendFac, dest_rgb: BlendFac, src_a: BlendFac, dest_a: BlendFac) {
		unsafe {
			self.ctx.blend_func_separate(src_rgb.into(), dest_rgb.into(), src_a.into(), dest_a.into());
		}
	}

	pub fn depth_func(&self, f: Cmp) {
		unsafe {
			self.ctx.depth_func(f.into());
		}
	}

	pub fn get_error(&self) -> Result<()> {

		unsafe {

			return match self.ctx.get_error() {
				glow::NO_ERROR => Ok(()),
				glow::INVALID_ENUM => Err(format!("opengl: invalid enum")),
				glow::INVALID_VALUE => Err(format!("opengl: invalid value")),
				glow::INVALID_OPERATION => Err(format!("opengl: invalid operation")),
				glow::STACK_OVERFLOW => Err(format!("opengl: stack overflow")),
				glow::STACK_UNDERFLOW => Err(format!("opengl: stack underflow")),
				glow::OUT_OF_MEMORY => Err(format!("opengl: out of memory")),
				glow::INVALID_FRAMEBUFFER_OPERATION => Err(format!("opengl: invalid framebuffer operation")),
				_ => Err(format!("opengl: unknown error")),
			};

		}

	}

	// TODO: move these to a RenderPass abstraction?
	pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
		unsafe {
			self.ctx.clear_color(r, g, b, a);
		}
	}

	pub fn clear(&self, buf: Surface) {
		unsafe {
			self.ctx.clear(buf.into());
		}
	}

	pub fn stencil<R>(&self, func: StencilFunc, ops: StencilOps, f: impl FnOnce() -> R) -> R {

		unsafe {
			self.ctx.stencil_func(func.cmp.into(), func.rf, func.mask);
			self.ctx.stencil_op(ops.sfail.into(), ops.dpfail.into(), ops.dppass.into());
			return f();
		}

	}

	pub fn cull_face(&self, face: Face) {
		unsafe {
			self.ctx.cull_face(face.into());
		}
	}

	pub fn front_face(&self, dir: CullMode) {
		unsafe {
			self.ctx.front_face(dir.into());
		}
	}

	pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
		unsafe {
			self.ctx.viewport(x, y, width, height);
		}
	}

	pub fn line_width(&self, w: f32) {
		unsafe {
			self.ctx.line_width(w);
		}
	}

}

