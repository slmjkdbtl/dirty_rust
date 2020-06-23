// wengwengweng

use std::marker::PhantomData;

use crate::*;
use gfx::*;
use res::shader::*;

/// Custom Shader. See [mod-level doc](index.html) for Usage.
#[derive(Clone, PartialEq)]
pub struct Shader<U: UniformLayout> {
	pipeline: Pipeline<Vertex, Uniform>,
	_custom_uniform: PhantomData<U>,
}

impl<U: UniformLayout> Shader<U> {

	/// create shader from only fragment code
	pub fn from_frag(ctx: &impl HasGL, frag: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			DEFAULT_VERT,
			&frag,
		);
	}

	/// create shader from only vertex code
	pub fn from_vert(ctx: &impl HasGL, vert: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			&vert,
			DEFAULT_FRAG,
		);
	}

	/// create shader from both vertex and fragment code
	pub fn from_vert_frag(ctx: &impl HasGL, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = TEMPLATE_VERT.replace("{{user}}", vert);
		let frag_src = TEMPLATE_FRAG.replace("{{user}}", frag);
		#[cfg(web)]
		let frag_src = format!("{}{}", "precision mediump float;", frag_src);

		return Ok(Self {
			pipeline: Pipeline::new(ctx, &vert_src, &frag_src)?,
			_custom_uniform: PhantomData,
		});

	}

	/// create default shader
	pub fn default(ctx: &impl HasGL) -> Result<Self> {
		return Self::from_vert_frag(ctx, DEFAULT_VERT, DEFAULT_FRAG);
	}

	/// free memory
	pub fn free(self) {
		self.pipeline.free();
	}

	pub(super) fn pipeline(&self) -> &Pipeline<Vertex, Uniform> {
		return &self.pipeline;
	}

}

