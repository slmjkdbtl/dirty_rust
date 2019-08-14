// wengwengweng

use std::rc::Rc;

use glow::Context;

use super::*;
use crate::Result;

type GLCtx = glow::native::Context;
type ProgramID = <GLCtx as Context>::Program;

#[derive(Clone, Debug)]
pub struct Program {
	ctx: Rc<GLCtx>,
	pub(super) id: ProgramID,
}

impl Program {

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
			};

			return Ok(program);

		}

	}

	pub fn send(&self, name: &str, value: impl UniformValue) {

		unsafe {

			self.bind();

			use UniformType::*;

			let loc = self.ctx.get_uniform_location(self.id, name);

			match value.get() {
				F1(f) => self.ctx.uniform_1_f32(loc, f),
				F2(f1, f2) => self.ctx.uniform_2_f32(loc, f1, f2),
				F3(f1, f2, f3) => self.ctx.uniform_3_f32(loc, f1, f2, f3),
				F4(f1, f2, f3, f4) => self.ctx.uniform_4_f32(loc, f1, f2, f3, f4),
				I1(i) => self.ctx.uniform_1_i32(loc, i),
				I2(i1, i2) => self.ctx.uniform_2_i32(loc, i1, i2),
				I3(i1, i2, i3) => self.ctx.uniform_3_i32(loc, i1, i2, i3),
				I4(i1, i2, i3, i4) => self.ctx.uniform_4_i32(loc, i1, i2, i3, i4),
				Mat4(a) => self.ctx.uniform_matrix_4_f32_slice(loc, false, &a),
			}

			self.unbind();

		}

	}

	pub fn with(&self, f: impl FnOnce()) {

		unsafe {
			self.ctx.use_program(Some(self.id));
			f();
			self.ctx.use_program(None);
		}

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
impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
// 			self.ctx.delete_program(self.id);
		}
	}
}

impl PartialEq for Program {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

