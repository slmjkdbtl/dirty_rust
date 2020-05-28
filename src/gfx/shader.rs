// wengwengweng

use std::marker::PhantomData;

use crate::*;
use gfx::*;
use res::shader::*;

/// Trait for Custom Uniform Data. See [mod-level doc](gfx) for Usage.
pub trait CustomUniform: Clone {
	fn values(&self) -> UniformValues {
		return hmap![];
	}
	fn textures(&self) -> Vec<&gfx::Texture> {
		return vec![];
	}
}

impl CustomUniform for () {}

/// Custom Shader. See [mod-level doc](gfx) for Usage.
#[derive(Clone, PartialEq)]
pub struct Shader<U: CustomUniform> {
	pipeline: Pipeline<Vertex, Uniform>,
	_custom_uniform: PhantomData<U>,
}

impl<U: CustomUniform> Shader<U> {

	pub(crate) fn from_pipeline(pipeline: Pipeline<Vertex, Uniform>) -> Self {
		return Self {
			pipeline: pipeline,
			_custom_uniform: PhantomData,
		};
	}

	pub fn from_frag(ctx: &impl HasGL, frag: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			DEFAULT_VERT,
			&frag,
		);
	}

	pub fn from_vert(ctx: &impl HasGL, vert: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			&vert,
			DEFAULT_FRAG,
		);
	}

	pub fn from_vert_frag(ctx: &impl HasGL, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = TEMPLATE_VERT.replace("{{user}}", vert);
		let frag_src = TEMPLATE_FRAG.replace("{{user}}", frag);
		#[cfg(web)]
		let frag_src = format!("{}{}", "precision mediump float;", frag_src);

		return Ok(Self::from_pipeline(Pipeline::new(ctx, &vert_src, &frag_src)?));

	}

	pub fn default(ctx: &impl HasGL) -> Result<Self> {
		return Self::from_vert_frag(ctx, DEFAULT_VERT, DEFAULT_FRAG);
	}

	pub fn free(self) {
		self.pipeline.free();
	}

	pub(crate) fn pipeline(&self) -> &Pipeline<Vertex, Uniform> {
		return &self.pipeline;
	}

}

