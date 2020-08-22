// wengwengweng

use super::*;

#[derive(Clone)]
pub(super) struct Pipeline<V: VertexLayout, U: UniformLayout> {
	handle: Rc<ProgramHandle>,
	gl: Rc<glow::Context>,
	_vertex_layout: PhantomData<V>,
	_uniform_layout: PhantomData<U>,
}

impl<V: VertexLayout, U: UniformLayout> Pipeline<V, U> {

	pub fn new(ctx: &impl GLCtx, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let handle = ProgramHandle::new(gl.clone())?;

			let vert_id = gl.create_shader(ShaderType::Vertex.as_glow())?;

			gl.shader_source(vert_id, vert_src);
			gl.compile_shader(vert_id);

			if !gl.get_shader_compile_status(vert_id) {
				return Err(format!("vert error: {}", gl.get_shader_info_log(vert_id).trim()));
			}

			let frag_id = gl.create_shader(ShaderType::Fragment.as_glow())?;

			gl.shader_source(frag_id, frag_src);
			gl.compile_shader(frag_id);

			if !gl.get_shader_compile_status(frag_id) {
				return Err(format!("frag error: {}", gl.get_shader_info_log(frag_id).trim()));
			}

			gl.attach_shader(handle.id(), vert_id);
			gl.attach_shader(handle.id(), frag_id);

			for (i, (name, _)) in V::attrs().iter().enumerate() {
				gl.bind_attrib_location(handle.id(), i as u32, name);
			}

			gl.link_program(handle.id());

			if !gl.get_program_link_status(handle.id()) {
				return Err(format!("glsl error: {}", gl.get_program_info_log(handle.id()).trim()));
			}

			gl.delete_shader(vert_id);
			gl.delete_shader(frag_id);

			let program = Self {
				gl: gl,
				handle: Rc::new(handle),
				_vertex_layout: PhantomData,
				_uniform_layout: PhantomData,
			};

			return Ok(program);

		}

	}

	pub fn draw(
		&self,
		prim: Primitive,
		vbuf: &VertexBuffer<V>,
		ibuf: &IndexBuffer,
		count: usize,
		uniform: &U,
	) {

		unsafe {

			self.gl.use_program(Some(self.handle.id()));
			vbuf.bind();
			bind_attrs::<V>(&self.gl);
			ibuf.bind();

			let mut tex_slots = vec![];

			// TODO: cache locations
			for (name, data) in uniform.data() {

				let loc = self.gl.get_uniform_location(self.handle.id(), name);

				if loc.is_some() {
					match data {
						UniformData::Float(f) => self.gl.uniform_1_f32(loc.as_ref(), f),
						UniformData::Vec2(f) => self.gl.uniform_2_f32(loc.as_ref(), f.x, f.y),
						UniformData::Vec3(f) => self.gl.uniform_3_f32(loc.as_ref(), f.x, f.y, f.z),
						UniformData::Vec4(f) => self.gl.uniform_4_f32(loc.as_ref(), f.x, f.y, f.z, f.w),
						UniformData::Int(i) => self.gl.uniform_1_i32(loc.as_ref(), i),
						UniformData::Mat4(m) => self.gl.uniform_matrix_4_f32_slice(loc.as_ref(), false, &m.as_arr()),
						UniformData::Texture(tex) => {
							self.gl.uniform_1_i32(loc.as_ref(), tex_slots.len() as i32);
							self.gl.active_texture(glow::TEXTURE0 + tex_slots.len() as u32);
							tex.bind();
							tex_slots.push(tex.clone());
						},
					}
				}

			}

			match prim {
				Primitive::Line(w) => self.gl.line_width(w),
				_ => {},
			}

			self.gl.draw_elements(prim.as_glow(), count as i32, glow::UNSIGNED_INT, 0);

			ibuf.unbind();
			vbuf.unbind();
			self.gl.use_program(None);

			for (i, tex) in tex_slots.into_iter().enumerate() {
				self.gl.active_texture(glow::TEXTURE0 + i as u32);
				tex.unbind();
			}

		}

	}

}

impl<V: VertexLayout, U: UniformLayout> PartialEq for Pipeline<V, U> {
	fn eq(&self, other: &Self) -> bool {
		return self.handle == other.handle;
	}
}

