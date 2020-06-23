// wengwengweng

use std::marker::PhantomData;

use crate::*;
use gfx::*;
use res::shader::*;

/// Trait for Custom Uniform Data. See [mod-level doc](index.html) for Usage.
pub trait CustomUniform: Clone {
	fn values(&self) -> Vec<(&'static str, &dyn IntoUniformValue)> {
		return vec![];
	}
	fn textures(&self) -> Vec<&Texture> {
		return vec![];
	}
}

impl CustomUniform for () {}

impl UniformData {
	pub fn from_uniform(uniform: &impl CustomUniform) -> Self {
		return Self {
			values: uniform
				.values()
				.into_iter()
				.map(|(n, v)| (n, v.into_uniform()))
				.collect(),
			textures: uniform
				.textures()
				.into_iter()
				.cloned()
				.collect(),
		};
	}
}

/// Custom Shader. See [mod-level doc](index.html) for Usage.
#[derive(Clone, PartialEq)]
pub struct Shader<U: CustomUniform> {
	pipeline: Pipeline<Vertex, Uniform>,
	_custom_uniform: PhantomData<U>,
}

impl<U: CustomUniform> Shader<U> {

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

