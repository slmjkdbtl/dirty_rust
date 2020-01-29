// wengwengweng

use std::rc::Rc;
use std::marker::PhantomData;

use glow::HasContext;

use super::*;
use crate::Result;

#[derive(Clone, Debug)]
pub struct Pipeline<V: VertexLayout, U: UniformLayout> {
	ctx: Rc<GLCtx>,
	program_id: ProgramID,
	attrs: VertexAttrGroup,
	vertex_layout: PhantomData<V>,
	uniform_layout: PhantomData<U>,
}

impl<V: VertexLayout, U: UniformLayout> Pipeline<V, U> {

	pub fn new(device: &Device, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let program_id = ctx.create_program()?;

			let vert_id = ctx.create_shader(ShaderType::Vertex.into())?;

			ctx.shader_source(vert_id, vert_src);
			ctx.compile_shader(vert_id);
			ctx.attach_shader(program_id, vert_id);

			if !ctx.get_shader_compile_status(vert_id) {
				return Err(format!("vert shader error: {}", ctx.get_shader_info_log(vert_id).trim()));
			}

			let frag_id = ctx.create_shader(ShaderType::Fragment.into())?;

			ctx.shader_source(frag_id, frag_src);
			ctx.compile_shader(frag_id);
			ctx.attach_shader(program_id, frag_id);

			if !ctx.get_shader_compile_status(frag_id) {
				return Err(format!("frag shader error: {}", ctx.get_shader_info_log(frag_id).trim()));
			}

			ctx.link_program(program_id);

			if !ctx.get_program_link_status(program_id) {
				return Err(format!("glsl error: {}", ctx.get_shader_info_log(program_id).trim()));
			}

			ctx.delete_shader(vert_id);
			ctx.delete_shader(frag_id);

			let program = Self {
				ctx: ctx,
				attrs: V::attrs(),
				program_id: program_id,
				vertex_layout: PhantomData,
				uniform_layout: PhantomData,
			};

			return Ok(program);

		}

	}

	pub fn send(&self, uniform: &U) {

		unsafe {

			use UniformValue::*;

			self.ctx.use_program(Some(self.program_id));

			for (name, value) in uniform.values() {

				// TODO: cache location
				let loc = self.ctx.get_uniform_location(self.program_id, name);

				match value.into_uniform() {
					F1(f) => self.ctx.uniform_1_f32(loc, f),
					F2(f) => self.ctx.uniform_2_f32(loc, f[0], f[1]),
					F3(f) => self.ctx.uniform_3_f32(loc, f[0], f[1], f[2]),
					F4(f) => self.ctx.uniform_4_f32(loc, f[0], f[1], f[2], f[3]),
					Mat4(a) => self.ctx.uniform_matrix_4_f32_slice(loc, false, &a),
				}

			}

			self.ctx.use_program(None);

		}

	}

	#[cfg(feature="gl3")]
	pub fn draw(
		&self,
		vao: Option<&VertexArray>,
		uniform: &U,
		count: u32,
		mode: Primitive,
	) {

		unsafe {

			self.send(&uniform);

			let textures = uniform.textures();

			self.ctx.use_program(Some(self.program_id));
			self.ctx.bind_vertex_array(vao.map(|v| v.id()));

			for (i, tex) in textures.iter().enumerate() {
				self.ctx.active_texture(glow::TEXTURE0 + i as u32);
				self.ctx.bind_texture(tex.r#type().into(), Some(tex.id()));
			}

			self.ctx.draw_elements(mode.into(), count as i32, glow::UNSIGNED_INT, 0);

			self.ctx.bind_vertex_array(None);
			self.ctx.use_program(None);

			for (i, tex) in textures.iter().enumerate() {
				self.ctx.active_texture(glow::TEXTURE0 + i as u32);
				self.ctx.bind_texture(tex.r#type().into(), None);
			}

		}

	}

	#[cfg(not(feature="gl3"))]
	pub fn draw(
		&self,
		vbuf: Option<&VertexBuffer<V>>,
		ibuf: Option<&IndexBuffer>,
		uniform: &U,
		count: u32,
		mode: Primitive,
	) {

		unsafe {

			self.send(&uniform);

			let textures = uniform.textures();

			self.ctx.use_program(Some(self.program_id));
			self.ctx.bind_buffer(glow::ARRAY_BUFFER, vbuf.map(|b| b.id()));

			if vbuf.is_some() {

				for attr in iter_attrs(&self.attrs) {

					if let Some(index) = self.ctx.get_attrib_location(self.program_id, &attr.name) {

						self.ctx.vertex_attrib_pointer_f32(
							index as u32,
							attr.size,
							glow::FLOAT,
							false,
							(V::STRIDE * mem::size_of::<f32>()) as i32,
							(attr.offset * mem::size_of::<f32>()) as i32,
						);

						self.ctx.enable_vertex_attrib_array(index as u32);

					}

				}

			}

			self.ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, ibuf.map(|b| b.id()));

			for (i, tex) in textures.iter().enumerate() {
				self.ctx.active_texture(glow::TEXTURE0 + i as u32);
				self.ctx.bind_texture(tex.r#type().into(), Some(tex.id()));
			}

			self.ctx.draw_elements(mode.into(), count as i32, glow::UNSIGNED_INT, 0);

			self.ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
			self.ctx.bind_buffer(glow::ARRAY_BUFFER, None);
			self.ctx.use_program(None);

			for (i, tex) in textures.iter().enumerate() {
				self.ctx.active_texture(glow::TEXTURE0 + i as u32);
				self.ctx.bind_texture(tex.r#type().into(), None);
			}

		}

	}

	pub fn drop(&self) {
		unsafe {
			self.ctx.delete_program(self.program_id);
		}
	}

}

impl<V: VertexLayout, U: UniformLayout> PartialEq for Pipeline<V, U> {
	fn eq(&self, other: &Self) -> bool {
		return self.program_id == other.program_id;
	}
}

