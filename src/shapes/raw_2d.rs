// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Raw2D<'a> {
	verts: Vec<Vertex2D>,
	indices: Vec<u32>,
	prim: gl::Primitive,
	tex: Option<&'a gfx::Texture>,
}

impl<'a> Raw2D<'a> {
	pub fn new(verts: &[Vertex2D], indices: &[u32]) -> Self {
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
}

pub fn raw_2d<'a>(verts: &[Vertex2D], indices: &[u32]) -> Raw2D<'a> {
	return Raw2D::new(verts, indices);
}

impl<'a> Drawable for Raw2D<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let tex = self.tex.unwrap_or(&ctx.empty_tex);
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
				tex: tex.clone(),
				custom: ctx.cur_custom_uniform_2d.clone(),
			},
		)?;

		return Ok(());

	}

}

