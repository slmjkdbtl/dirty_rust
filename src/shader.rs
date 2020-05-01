// wengwengweng

// custom shader inputs:
//
// varying vec3 v_pos;
// varying vec3 v_normal;
// varying vec2 v_uv;
// varying vec4 v_color;
//
// uniform mat4 u_model;
// uniform mat4 u_view;
// uniform mat4 u_proj;
// uniform sampler2D u_tex;
// uniform vec4 u_color;
//
// vec4 default_pos();
// vec4 default_color();

use std::rc::Rc;
use std::marker::PhantomData;

use crate::*;
use math::*;
use gfx::*;
use res::shader::*;

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

#[derive(Clone, PartialEq)]
pub struct Shader<U: CustomUniform> {
	gl_pipeline: Rc<gl::Pipeline<Vertex, Uniform>>,
	uniform: PhantomData<U>,
}

impl<U: CustomUniform> Shader<U> {

	pub(crate) fn from_gl_pipeline(gl_pipeline: gl::Pipeline<Vertex, Uniform>) -> Self {
		return Self {
			gl_pipeline: Rc::new(gl_pipeline),
			uniform: PhantomData,
		};
	}

	pub fn from_frag(ctx: &impl gfx::GfxCtx, frag: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			DEFAULT_VERT,
			&frag,
		);
	}

	pub fn from_vert(ctx: &impl gfx::GfxCtx, vert: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			&vert,
			DEFAULT_FRAG,
		);
	}

	pub fn from_vert_frag(ctx: &impl gfx::GfxCtx, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = TEMPLATE_VERT.replace("###REPLACE###", vert);
		let frag_src = TEMPLATE_FRAG.replace("###REPLACE###", frag);

		return Ok(Self::from_gl_pipeline(gl::Pipeline::new(ctx.device(), &vert_src, &frag_src)?));

	}

	pub(crate) fn gl_pipeline(&self) -> &gl::Pipeline<Vertex, Uniform> {
		return &self.gl_pipeline;
	}

}

