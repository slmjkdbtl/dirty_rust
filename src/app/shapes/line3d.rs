// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Line3D {
	p1: Vec3,
	p2: Vec3,
	color: Color,
	width: f32,
}

pub fn line3d(p1: Vec3, p2: Vec3) -> Line3D {
	return Line3D::from(p1, p2);
}

impl Line3D {
	pub fn from(p1: Vec3, p2: Vec3) -> Self {
		return Self {
			p1: p1,
			p2: p2,
			color: rgba!(),
			width: 1.0,
		};
	}
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
}

impl Drawable for Line3D {

	// TODO: deal with out of bound
	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut verts = Vec::with_capacity(2 * gfx::Vertex3D::STRIDE);

		gfx::Vertex3D {
			pos: self.p1,
			normal: vec3!(0),
			color: self.color,
			uv: vec2!(0),
		}.push(&mut verts);

		gfx::Vertex3D {
			pos: self.p2,
			normal: vec3!(0),
			color: self.color,
			uv: vec2!(0),
		}.push(&mut verts);

		ctx.renderer_3d.push(
			gl::Primitive::Line,
			&verts,
			&[0, 1],
			&ctx.cur_pipeline_3d,
			&gfx::Uniform3D {
				proj: ctx.proj,
				view: ctx.view,
				model: ctx.transform,
				color: rgba!(),
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_3d.clone(),
			},
		)?;

// 		let p1 = ctx.to_sc(self.p1);
// 		let p2 = ctx.to_sc(self.p2);

// 		ctx.draw(
// 			&line(p1, p2)
// 				.color(self.color)
// 		)?;

		return Ok(());

	}

}

