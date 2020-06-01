// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Mesh<'a> {
	mesh: &'a gfx::Mesh,
	prim: Primitive,
	tex: Option<&'a gfx::Texture>,
	color: Color,
}

impl<'a> Mesh<'a> {
	pub fn new(m: &'a gfx::Mesh) -> Self {
		return Self {
			mesh: m,
			prim: Primitive::Triangle,
			tex: None,
			color: rgba!(1),
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
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
}

pub fn mesh<'a>(m: &'a gfx::Mesh) -> Mesh<'a> {
	return Mesh::new(m);
}

impl<'a> Drawable for Mesh<'a> {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		let tex = self.tex.unwrap_or(&ctx.empty_tex);

		ctx.cur_pipeline.draw(
			self.prim,
			self.mesh.vbuf(),
			self.mesh.ibuf(),
			self.mesh.count(),
			&gfx::Uniform {
				proj: ctx.proj,
				view: ctx.view,
				model: ctx.transform,
				color: self.color,
				tex: tex.clone(),
				custom: ctx.cur_custom_uniform.clone(),
			},
		);

		ctx.draw_calls += 1;

		return Ok(());

	}

}

