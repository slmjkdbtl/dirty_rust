// wengwengweng

use std::rc::Rc;
use std::marker::PhantomData;

use crate::*;
use math::*;
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

impl IntoUniformValue for Vec2 {
	fn into_uniform(&self) -> gl::UniformValue {
		return gl::UniformValue::F2(self.as_arr());
	}
}

impl IntoUniformValue for Vec3 {
	fn into_uniform(&self) -> gl::UniformValue {
		return gl::UniformValue::F3(self.as_arr());
	}
}

impl IntoUniformValue for Vec4 {
	fn into_uniform(&self) -> gl::UniformValue {
		return gl::UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Color {
	fn into_uniform(&self) -> gl::UniformValue {
		return gl::UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Quad {
	fn into_uniform(&self) -> gl::UniformValue {
		return gl::UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Mat4 {
	fn into_uniform(&self) -> gl::UniformValue {
		return gl::UniformValue::Mat4(self.as_arr());
	}
}

impl IntoUniformValue for std::time::Duration {
	fn into_uniform(&self) -> gl::UniformValue {
		return gl::UniformValue::F1(self.as_secs_f32());
	}
}

/// Custom Shader. See [mod-level doc](gfx) for Usage.
#[derive(Clone, PartialEq)]
pub struct Shader<U: CustomUniform> {
	gl_pipeline: Rc<gl::Pipeline<Vertex, Uniform>>,
	_custom_uniform: PhantomData<U>,
}

impl<U: CustomUniform> Shader<U> {

	pub(crate) fn from_gl_pipeline(gl_pipeline: gl::Pipeline<Vertex, Uniform>) -> Self {
		return Self {
			gl_pipeline: Rc::new(gl_pipeline),
			_custom_uniform: PhantomData,
		};
	}

	pub fn from_frag(ctx: &impl HasGLDevice, frag: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			DEFAULT_VERT,
			&frag,
		);
	}

	pub fn from_vert(ctx: &impl HasGLDevice, vert: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			&vert,
			DEFAULT_FRAG,
		);
	}

	pub fn from_vert_frag(ctx: &impl HasGLDevice, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = TEMPLATE_VERT.replace("{{user}}", vert);
		let frag_src = TEMPLATE_FRAG.replace("{{user}}", frag);
		#[cfg(web)]
		let frag_src = format!("{}{}", "precision mediump float;", frag_src);

		return Ok(Self::from_gl_pipeline(gl::Pipeline::new(ctx.device(), &vert_src, &frag_src)?));

	}

	pub fn default(ctx: &impl HasGLDevice) -> Result<Self> {
		return Self::from_vert_frag(ctx, DEFAULT_VERT, DEFAULT_FRAG);
	}

	pub(crate) fn gl_pipeline(&self) -> &gl::Pipeline<Vertex, Uniform> {
		return &self.gl_pipeline;
	}

}

