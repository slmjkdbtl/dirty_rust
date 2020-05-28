// wengwengweng

use std::mem;
use std::rc::Rc;
use std::marker::PhantomData;

use glow::HasContext;

use super::*;
use crate::Result;

// #[derive(Clone, Copy, Debug)]
// pub struct BlendDesc {
// 	pub src: BlendFac,
// 	pub dest: BlendFac,
// 	pub op: BlendOp,
// }

// #[derive(Clone, Copy, Debug)]
// pub struct BlendState {
// 	pub color: BlendDesc,
// 	pub alpha: BlendDesc,
// }

// #[derive(Clone, Copy, Debug)]
// pub struct StencilState {
// 	pub cmp: Cmp,
// 	pub fail_op: StencilOp,
// 	pub depth_fail_op: StencilOp,
// 	pub pass_op: StencilOp,
// }

// #[derive(Clone)]
// pub struct RenderState<'a, U: UniformLayout> {
// 	pub prim: Primitive,
// 	pub uniform: &'a U,
// 	pub blend: BlendState,
// 	pub frame_buffer: Option<&'a Framebuffer>,
// }

#[derive(Clone, Debug)]
pub struct Pipeline<V: VertexLayout, U: UniformLayout> {
	gl: Rc<glow::Context>,
	program_id: ProgramID,
	attrs: VertexAttrGroup,
	_vertex_layout: PhantomData<V>,
	_uniform_layout: PhantomData<U>,
}

impl<V: VertexLayout, U: UniformLayout> Pipeline<V, U> {

	pub fn new(ctx: &impl HasGL, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let program_id = gl.create_program()?;

			let vert_id = gl.create_shader(ShaderType::Vertex.into())?;

			gl.shader_source(vert_id, vert_src);
			gl.compile_shader(vert_id);
			gl.attach_shader(program_id, vert_id);

			if !gl.get_shader_compile_status(vert_id) {
				return Err(format!("vert error: {}", gl.get_shader_info_log(vert_id).trim()));
			}

			let frag_id = gl.create_shader(ShaderType::Fragment.into())?;

			gl.shader_source(frag_id, frag_src);
			gl.compile_shader(frag_id);
			gl.attach_shader(program_id, frag_id);

			if !gl.get_shader_compile_status(frag_id) {
				return Err(format!("frag error: {}", gl.get_shader_info_log(frag_id).trim()));
			}

			gl.link_program(program_id);

			if !gl.get_program_link_status(program_id) {
// 				return Err(format!("glsl error: {}", gl.get_shader_info_log(program_id).trim()));
			}

			gl.delete_shader(vert_id);
			gl.delete_shader(frag_id);

			let program = Self {
				gl: gl,
				attrs: V::attrs(),
				program_id: program_id,
				_vertex_layout: PhantomData,
				_uniform_layout: PhantomData,
			};

			return Ok(program);

		}

	}

	fn send(&self, uniform: &U) {

		unsafe {

			use UniformValue::*;

			self.gl.use_program(Some(self.program_id));

			for (name, value) in uniform.values() {

				let loc = self.gl.get_uniform_location(self.program_id, name);

				if loc.is_some() {
					match value.into_uniform() {
						F1(f) => self.gl.uniform_1_f32(loc, f),
						F2(f) => self.gl.uniform_2_f32(loc, f[0], f[1]),
						F3(f) => self.gl.uniform_3_f32(loc, f[0], f[1], f[2]),
						F4(f) => self.gl.uniform_4_f32(loc, f[0], f[1], f[2], f[3]),
						Mat4(a) => self.gl.uniform_matrix_4_f32_slice(loc, false, &a),
					}
				}

			}

			self.gl.use_program(None);

		}

	}

	pub fn draw(
		&self,
		vbuf: &VertexBuffer<V>,
		ibuf: &IndexBuffer,
		uniform: &U,
		count: u32,
		prim: Primitive,
	) {

		unsafe {

			self.send(&uniform);

			let textures = uniform.textures();

			self.gl.use_program(Some(self.program_id));
			vbuf.bind();

			for attr in iter_attrs(&self.attrs) {

				if let Some(index) = self.gl.get_attrib_location(self.program_id, &attr.name) {

					self.gl.vertex_attrib_pointer_f32(
						index as u32,
						attr.size,
						glow::FLOAT,
						false,
						mem::size_of::<V>() as i32,
						(attr.offset * mem::size_of::<f32>()) as i32,
					);

					self.gl.enable_vertex_attrib_array(index as u32);

				}

			}

			ibuf.bind();

			for (i, tex) in textures.iter().enumerate() {
				self.gl.active_texture(glow::TEXTURE0 + i as u32);
				tex.bind();
			}

			match prim {
				Primitive::Line(w) => self.gl.line_width(w),
				_ => {},
			}

			self.gl.draw_elements(prim.into(), count as i32, glow::UNSIGNED_INT, 0);

			ibuf.unbind();
			vbuf.unbind();
			self.gl.use_program(None);

			for (i, tex) in textures.iter().enumerate() {
				self.gl.active_texture(glow::TEXTURE0 + i as u32);
				tex.unbind();
			}

		}

	}

	pub fn free(self) {
		unsafe {
			self.gl.delete_program(self.program_id);
		}
	}

}

impl<V: VertexLayout, U: UniformLayout> PartialEq for Pipeline<V, U> {
	fn eq(&self, other: &Self) -> bool {
		return self.program_id == other.program_id;
	}
}

