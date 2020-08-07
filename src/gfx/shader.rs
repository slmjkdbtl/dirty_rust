// wengwengweng

use super::*;

/// Custom Shader. See [mod-level doc](index.html) for Usage.
#[derive(Clone, PartialEq)]
pub struct Shader<U: UniformLayout> {
	pipeline: Pipeline<Vertex, Uniform>,
	_custom_uniform: PhantomData<U>,
}

impl<U: UniformLayout> Shader<U> {

	/// create shader from only fragment code
	pub fn from_frag(ctx: &impl GLCtx, frag: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			shaders::DEFAULT_VERT,
			&frag,
		);
	}

	/// create shader from only vertex code
	pub fn from_vert(ctx: &impl GLCtx, vert: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			&vert,
			shaders::DEFAULT_FRAG,
		);
	}

	/// create shader from both vertex and fragment code
	pub fn from_vert_frag(ctx: &impl GLCtx, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = shaders::TEMPLATE_VERT.replace("{{user}}", vert);
		let frag_src = shaders::TEMPLATE_FRAG.replace("{{user}}", frag);
		#[cfg(any(web, mobile))]
		let frag_src = format!("{}{}", "precision mediump float;", frag_src);

		return Ok(Self {
			pipeline: Pipeline::new(ctx, &vert_src, &frag_src)?,
			_custom_uniform: PhantomData,
		});

	}

	/// create default shader
	pub fn default(ctx: &impl GLCtx) -> Result<Self> {
		return Self::from_vert_frag(ctx, shaders::DEFAULT_VERT, shaders::DEFAULT_FRAG);
	}

	pub(super) fn pipeline(&self) -> &Pipeline<Vertex, Uniform> {
		return &self.pipeline;
	}

}

