// wengwengweng

use super::*;
use gfx::MeshData;

#[derive(Clone)]
pub struct Raw<'a> {
	verts: Vec<Vertex>,
	indices: Vec<u32>,
	prim: gl::Primitive,
	tex: Option<&'a gfx::Texture>,
}

impl<'a> Raw<'a> {
	pub fn new(verts: &[Vertex], indices: &[u32]) -> Self {
		return Self {
			verts: verts.to_vec(),
			indices: indices.to_vec(),
			prim: gl::Primitive::Triangle,
			tex: None,
		};
	}
	pub fn from_meshdata(m: &'a MeshData) -> Self {
		return Self::new(&m.vertices, &m.indices);
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

pub fn raw<'a>(verts: &[Vertex], indices: &[u32]) -> Raw<'a> {
	return Raw::new(verts, indices);
}

impl<'a> Drawable for Raw<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let tex = self.tex.unwrap_or(&ctx.empty_tex);
		let mut verts = Vec::with_capacity(self.verts.len() * gfx::Vertex::STRIDE);

		for p in &self.verts {
			p.push(&mut verts);
		}

		ctx.renderer.push(
			self.prim,
			&verts,
			&self.indices,
			&ctx.cur_pipeline,
			&gfx::Uniform {
				proj: ctx.proj,
				view: ctx.view,
				model: ctx.transform,
				color: rgba!(1),
				tex: tex.clone(),
				custom: ctx.cur_custom_uniform.clone(),
			},
		)?;

		return Ok(());

	}

}

