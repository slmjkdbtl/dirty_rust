// wengwengweng

use std::rc::Rc;
use std::marker::PhantomData;

use glow::Context;

use super::*;
use crate::Result;

#[derive(Clone, Debug)]
pub struct Program<U: UniformInterface> {
	ctx: Rc<GLCtx>,
	pub(super) id: ProgramID,
	uniform_interface: PhantomData<U>,
}

impl<U: UniformInterface> Program<U> {

	pub fn new(device: &Device, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let program_id = ctx.create_program()?;

			let vert_id = ctx.create_shader(ShaderType::Vertex.into())?;

			ctx.shader_source(vert_id, vert_src);
			ctx.compile_shader(vert_id);
			ctx.attach_shader(program_id, vert_id);

			if !ctx.get_shader_compile_status(vert_id) {
				return Err(Error::OpenGL(ctx.get_shader_info_log(vert_id)));
			}

			let frag_id = ctx.create_shader(ShaderType::Fragment.into())?;

			ctx.shader_source(frag_id, frag_src);
			ctx.compile_shader(frag_id);
			ctx.attach_shader(program_id, frag_id);

			if !ctx.get_shader_compile_status(frag_id) {
				return Err(Error::OpenGL(ctx.get_shader_info_log(frag_id)));
			}

			ctx.link_program(program_id);

			if !ctx.get_program_link_status(program_id) {
				return Err(Error::OpenGL(ctx.get_program_info_log(program_id)));
			}

			ctx.delete_shader(vert_id);
			ctx.delete_shader(frag_id);

			let program = Self {
				ctx: ctx,
				id: program_id,
				uniform_interface: PhantomData,
			};

			return Ok(program);

		}

	}

	pub fn send(&self, uniform: &impl UniformInterface) {

		unsafe {

			use UniformType::*;

			self.bind();

			for v in uniform.values() {

				let loc = self.ctx.get_uniform_location(self.id, v.0);

				match v.1 {
					F1(f) => self.ctx.uniform_1_f32(loc, f),
					F2(f) => self.ctx.uniform_2_f32(loc, f[0], f[1]),
					F3(f) => self.ctx.uniform_3_f32(loc, f[0], f[1], f[2]),
					F4(f) => self.ctx.uniform_4_f32(loc, f[0], f[1], f[2], f[3]),
					Mat4(a) => self.ctx.uniform_matrix_4_f32_slice(loc, false, &a),
				}

			}

			self.unbind();

		}

	}

	pub fn with<R>(&self, f: impl FnOnce() -> R) -> R {

		self.bind();
		let r = f();
		self.unbind();

		return r;

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.ctx.use_program(Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.ctx.use_program(None);
		}
	}

}

// TODO
impl<U: UniformInterface> Drop for Program<U> {
	fn drop(&mut self) {
		unsafe {
// 			self.ctx.delete_program(self.id);
		}
	}
}

impl<U: UniformInterface> PartialEq for Program<U> {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

