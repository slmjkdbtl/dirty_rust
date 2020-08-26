// wengwengweng

use super::*;

#[derive(Clone)]
pub(super) struct Pipeline<V: VertexLayout, U: UniformLayout> {
	handle: Rc<ProgramHandle>,
	_vertex_layout: PhantomData<V>,
	_uniform_layout: PhantomData<U>,
}

impl<V: VertexLayout, U: UniformLayout> Pipeline<V, U> {

	pub fn new(ctx: &impl GLCtx, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let handle = ProgramHandle::new(ctx.gl())?;
			let gl = handle.ctx();

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

			return Ok(Self {
				handle: Rc::new(handle),
				_vertex_layout: PhantomData,
				_uniform_layout: PhantomData,
			});

		}

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.handle.ctx().use_program(Some(self.handle.id()));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.handle.ctx().use_program(None);
		}
	}

	pub(super) fn loc(&self, name: &'static str) -> Option<glow::UniformLocation> {
		unsafe {
			return self.handle.ctx().get_uniform_location(self.handle.id(), name);
		}
	}

}

impl<V: VertexLayout, U: UniformLayout> PartialEq for Pipeline<V, U> {
	fn eq(&self, other: &Self) -> bool {
		return self.handle == other.handle;
	}
}

