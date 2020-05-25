// wengwengweng

use super::*;
use gfx::MeshData;

#[derive(Clone)]
pub struct Raw<'a> {
	verts: &'a [Vertex],
	indices: &'a [u32],
	prim: gl::Primitive,
	tex: Option<&'a gfx::Texture>,
	color: Color,
	transformed: bool,
}

impl<'a> Raw<'a> {
	pub fn new(verts: &'a [Vertex], indices: &'a [u32]) -> Self {
		return Self {
			verts,
			indices,
			prim: gl::Primitive::Triangle,
			tex: None,
			color: rgba!(1),
			transformed: false,
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
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
	pub fn transformed(mut self) -> Self {
		self.transformed = true;
		return self;
	}
}

pub fn raw<'a>(verts: &'a [Vertex], indices: &'a [u32]) -> Raw<'a> {
	return Raw::new(verts, indices);
}

impl<'a> Drawable for Raw<'a> {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		let tex = self.tex.unwrap_or(&ctx.empty_tex);

		ctx.renderer.push(
			self.prim,
			&self.verts,
			&self.indices,
			&ctx.cur_pipeline,
			&gfx::Uniform {
				proj: ctx.proj,
				view: ctx.view,
				model: if self.transformed {
					mat4!()
				} else {
					ctx.transform
				},
				color: self.color,
				tex: tex.clone(),
				custom: ctx.cur_custom_uniform.clone(),
			},
		)?;

		return Ok(());

	}

}

