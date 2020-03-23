// wengwengweng

use std::rc::Rc;

use crate::*;
use gfx::*;

#[derive(Clone)]
pub struct Skybox {
	gl_tex: gl::CubemapTexture,
	mesh: Rc<gl::Mesh<VertexCubemap, UniformCubemap>>,
}

impl Skybox {

	pub fn from_bytes(
		ctx: &impl gfx::GfxCtx,
		right: &[u8],
		left: &[u8],
		top: &[u8],
		bottom: &[u8],
		front: &[u8],
		back: &[u8],
	) -> Result<Self> {

		let mesh = gl::Mesh::from_shape(ctx.device(), CubemapShape)?;

		let right = img::Image::from_bytes(right)?;
		let left = img::Image::from_bytes(left)?;
		let top = img::Image::from_bytes(top)?;
		let bottom = img::Image::from_bytes(bottom)?;
		let front = img::Image::from_bytes(front)?;
		let back = img::Image::from_bytes(back)?;

		let tex = gl::CubemapTexture::from(
			ctx.device(),
			right.width(),
			right.height(),
			&right.into_raw(),
			&left.into_raw(),
			&top.into_raw(),
			&bottom.into_raw(),
			&front.into_raw(),
			&back.into_raw(),
		)?;

		return Ok(Self {
			mesh: Rc::new(mesh),
			gl_tex: tex,
		});

	}

	pub(crate) fn gl_tex(&self) -> &gl::CubemapTexture {
		return &self.gl_tex;
	}

}

