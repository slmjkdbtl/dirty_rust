// wengwengweng

use std::rc::Rc;
use std::marker::PhantomData;

use crate::*;
use super::*;
use super::gfx::*;

/// custom uniform
pub trait Uniform: Clone {
	fn values(&self) -> UniformValues {
		return hmap![];
	}
	fn textures(&self) -> Vec<&gfx::Texture> {
		return vec![];
	}
}

impl Uniform for () {}

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

/// custom shader for 2D
#[derive(Clone, PartialEq)]
pub struct Shader2D<U: Uniform> {
	gl_pipeline: Rc<gl::Pipeline<Vertex2D, Uniform2D>>,
	uniform: PhantomData<U>,
}

impl<U: Uniform> Shader2D<U> {

	pub(super) fn from_gl_pipeline(gl_pipeline: gl::Pipeline<Vertex2D, Uniform2D>) -> Self {
		return Self {
			gl_pipeline: Rc::new(gl_pipeline),
			uniform: PhantomData,
		};
	}

	/// custom fragment shader
	pub fn from_frag(ctx: &Ctx, frag: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			res::DEFAULT_2D_VERT,
			&frag,
		);
	}

	/// custom vertex shader
	pub fn from_vert(ctx: &Ctx, vert: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			&vert,
			res::DEFAULT_2D_FRAG,
		);
	}

	/// custom vertex & fragment shader
	pub fn from_vert_frag(ctx: &Ctx, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = res::TEMPLATE_2D_VERT.replace("###REPLACE###", vert);
		let frag_src = res::TEMPLATE_2D_FRAG.replace("###REPLACE###", frag);

		return Ok(Self::from_gl_pipeline(gl::Pipeline::new(&ctx.gl, &vert_src, &frag_src)?));

	}

	pub(super) fn gl_pipeline(&self) -> &gl::Pipeline<Vertex2D, Uniform2D> {
		return &self.gl_pipeline;
	}

}

/// custom shader for 3D
#[derive(Clone, PartialEq)]
pub struct Shader3D<U: Uniform> {
	gl_pipeline: Rc<gl::Pipeline<Vertex3D, Uniform3D>>,
	uniform: PhantomData<U>,
}

impl<U: Uniform> Shader3D<U> {

	pub(super) fn from_gl_pipeline(gl_pipeline: gl::Pipeline<Vertex3D, Uniform3D>) -> Self {
		return Self {
			gl_pipeline: Rc::new(gl_pipeline),
			uniform: PhantomData,
		};
	}

	/// custom fragment shader
	pub fn from_frag(ctx: &Ctx, frag: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			res::DEFAULT_3D_VERT,
			&frag,
		);
	}

	/// custom vertex shader
	pub fn from_vert(ctx: &Ctx, vert: &str) -> Result<Self> {
		return Self::from_vert_frag(
			ctx,
			&vert,
			res::DEFAULT_3D_FRAG,
		);
	}

	/// custom vertex & fragment shader
	pub fn from_vert_frag(ctx: &Ctx, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = res::TEMPLATE_3D_VERT.replace("###REPLACE###", vert);
		let frag_src = res::TEMPLATE_3D_FRAG.replace("###REPLACE###", frag);

		return Ok(Self::from_gl_pipeline(gl::Pipeline::new(&ctx.gl, &vert_src, &frag_src)?));

	}

	pub(super) fn gl_pipeline(&self) -> &gl::Pipeline<Vertex3D, Uniform3D> {
		return &self.gl_pipeline;
	}

}

