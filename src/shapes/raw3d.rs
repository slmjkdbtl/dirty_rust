// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Raw3D<'a> {
	verts: Vec<Vertex3D>,
	indices: Vec<u32>,
	prim: gl::Primitive,
	tex: Option<&'a gfx::Texture>,
}

impl<'a> Raw3D<'a> {
	pub fn new(verts: &[Vertex3D], indices: &[u32]) -> Self {
		return Self {
			verts: verts.to_vec(),
			indices: indices.to_vec(),
			prim: gl::Primitive::Triangle,
			tex: None,
		};
	}
	pub fn texture(mut self, tex: &'a gfx::Texture) -> Self {
		self.tex = Some(tex);
		return self;
	}
	pub fn prim(mut self, prim: gfx::Primitive) -> Self {
		self.prim = prim;
		return self;
	}
}

pub fn raw3d<'a>(verts: &[Vertex3D], indices: &[u32]) -> Raw3D<'a> {
	return Raw3D::new(verts, indices);
}

impl<'a> Drawable for Raw3D<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let tex = self.tex.unwrap_or(&ctx.empty_tex);
		let mut verts = Vec::with_capacity(self.verts.len() * gfx::Vertex3D::STRIDE);

		for p in &self.verts {
			p.push(&mut verts);
		}

		ctx.renderer_3d.push(
			self.prim,
			&verts,
			&self.indices,
			&ctx.cur_pipeline_3d,
			&gfx::Uniform3D {
				proj: ctx.proj,
				view: ctx.view,
				model: ctx.transform,
				color: rgba!(1),
				tex: tex.clone(),
				custom: ctx.cur_custom_uniform_3d.clone(),
			},
		)?;

		return Ok(());

	}

}


