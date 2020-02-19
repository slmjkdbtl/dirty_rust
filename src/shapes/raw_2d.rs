// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Raw2D {
	verts: Vec<Vertex2D>,
	indices: Vec<u32>,
	prim: gl::Primitive,
}

impl Raw2D {
	pub fn new(verts: &[Vertex2D], indices: &[u32]) -> Self {
		return Self {
			verts: verts.to_vec(),
			indices: indices.to_vec(),
			prim: gl::Primitive::Triangle,
		};
	}
}

pub fn raw_2d(verts: &[Vertex2D], indices: &[u32]) -> Raw2D {
	return Raw2D::new(verts, indices);
}

impl Drawable for Raw2D {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut verts = Vec::with_capacity(self.verts.len() * gfx::Vertex2D::STRIDE);

		for p in &self.verts {
			p.push(&mut verts);
		}

		ctx.renderer_2d.push(
			self.prim,
			&verts,
			&self.indices,
			&ctx.cur_pipeline_2d,
			&gfx::Uniform2D {
				proj: ctx.proj,
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_2d.clone(),
			},
		)?;

		return Ok(());

	}

}

