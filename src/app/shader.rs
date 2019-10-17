// wengwengweng

use std::rc::Rc;
use std::marker::PhantomData;

use crate::*;
use super::*;
use super::gfx::*;

pub trait Uniform: Clone {
	fn values(&self) -> UniformValues;
}

impl Uniform for () {
	fn values(&self) -> UniformValues {
		return vec![];
	}
}

#[derive(Clone, PartialEq)]
pub struct Shader2D<U: Uniform> {
	pub(super) handle: Rc<gl::Pipeline<Vertex2D, Uniform2D>>,
	uniform: PhantomData<U>,
}

impl<U: Uniform> Shader2D<U> {

	pub(super) fn from_handle(handle: gl::Pipeline<Vertex2D, Uniform2D>) -> Self {
		return Self {
			handle: Rc::new(handle),
			uniform: PhantomData,
		};
	}

	pub fn from_frag(ctx: &Ctx, frag: &str) -> Result<Self> {

		return Self::from_vert_frag(
			ctx,
			res::DEFAULT_2D_VERT,
			&frag,
		);

	}

	pub fn from_vert(ctx: &Ctx, vert: &str) -> Result<Self> {

		return Self::from_vert_frag(
			ctx,
			&vert,
			res::DEFAULT_2D_FRAG,
		);

	}

	pub fn from_vert_frag(ctx: &Ctx, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = res::TEMPLATE_2D_VERT.replace("###REPLACE###", vert);
		let frag_src = res::TEMPLATE_2D_FRAG.replace("###REPLACE###", frag);

		return Ok(Self::from_handle(gl::Pipeline::new(&ctx.gl, &vert_src, &frag_src)?));

	}

}

#[derive(Clone, PartialEq)]
pub struct Shader3D<U: Uniform> {
	pub(super) handle: Rc<gl::Pipeline<Vertex3D, Uniform3D>>,
	uniform: PhantomData<U>,
}

impl<U: Uniform> Shader3D<U> {

	pub(super) fn from_handle(handle: gl::Pipeline<Vertex3D, Uniform3D>) -> Self {
		return Self {
			handle: Rc::new(handle),
			uniform: PhantomData,
		};
	}

	pub fn from_frag(ctx: &Ctx, frag: &str) -> Result<Self> {

		return Self::from_vert_frag(
			ctx,
			res::DEFAULT_3D_VERT,
			&frag,
		);

	}

	pub fn from_vert(ctx: &Ctx, vert: &str) -> Result<Self> {

		return Self::from_vert_frag(
			ctx,
			&vert,
			res::DEFAULT_3D_FRAG,
		);

	}

	pub fn from_vert_frag(ctx: &Ctx, vert: &str, frag: &str) -> Result<Self> {

		let vert_src = res::TEMPLATE_3D_VERT.replace("###REPLACE###", vert);
		let frag_src = res::TEMPLATE_3D_FRAG.replace("###REPLACE###", frag);

		return Ok(Self::from_handle(gl::Pipeline::new(&ctx.gl, &vert_src, &frag_src)?));

	}

}

