// wengwengweng

use super::*;

#[derive(Clone)]
pub(super) struct Pipeline<V: VertexLayout, U: UniformLayout> {
	handle: Rc<ProgramHandle>,
	gl: Rc<glow::Context>,
	attrs: VertexAttrGroup,
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
				attrs: V::attrs(),
				_vertex_layout: PhantomData,
				_uniform_layout: PhantomData,
			};

			return Ok(program);

		}

	}

	fn send_uniforms(&self, uniform: &U) {

		unsafe {

			use UniformValue::*;

			self.gl.use_program(Some(self.handle.id()));

			// TODO: cache locations
			for (name, value) in uniform.values() {

				let loc = self.gl.get_uniform_location(self.handle.id(), name);

				if loc.is_some() {
					match value {
						Float(f) => self.gl.uniform_1_f32(loc.as_ref(), f),
						Vec2(f) => self.gl.uniform_2_f32(loc.as_ref(), f.x, f.y),
						Vec3(f) => self.gl.uniform_3_f32(loc.as_ref(), f.x, f.y, f.z),
						Vec4(f) => self.gl.uniform_4_f32(loc.as_ref(), f.x, f.y, f.z, f.w),
						Int(i) => self.gl.uniform_1_i32(loc.as_ref(), i),
						Mat4(m) => self.gl.uniform_matrix_4_f32_slice(loc.as_ref(), false, &m.as_arr()),
					}
				}

			}

			self.gl.use_program(None);

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

			self.send_uniforms(&uniform);

			let textures = uniform.textures();

			self.gl.use_program(Some(self.handle.id()));
			vbuf.bind();

			for (i, attr) in iter_attrs(&self.attrs).enumerate() {

				self.gl.vertex_attrib_pointer_f32(
					i as u32,
					attr.size,
					glow::FLOAT,
					false,
					mem::size_of::<V>() as i32,
					(attr.offset * mem::size_of::<f32>()) as i32,
				);

				self.gl.enable_vertex_attrib_array(i as u32);

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

			self.gl.draw_elements(prim.as_glow(), count as i32, glow::UNSIGNED_INT, 0);

			ibuf.unbind();
			vbuf.unbind();
			self.gl.use_program(None);

			for (i, tex) in textures.iter().enumerate() {
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

