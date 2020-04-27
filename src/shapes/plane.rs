// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Plane {
	pos: Vec3,
	width: f32,
	height: f32,
	normal: Vec3,
	color: Color,
	grid_size: Option<Vec2>,
}

pub fn plane(pos: Vec3, normal: Vec3, w: f32, h: f32) -> Plane {
	return Plane::new(pos, normal, w, h);
}

impl Plane {
	pub fn new(pos: Vec3, normal: Vec3, w: f32, h: f32) -> Self {
		return Self {
			pos: pos,
			normal: normal,
			width: w,
			height: h,
			color: rgba!(1),
			grid_size: None,
		};
	}
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
	pub fn grid_size(mut self, s: Vec2) -> Self {
		self.grid_size = Some(s);
		return self;
	}
}

impl Drawable for Plane {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut verts = Vec::with_capacity(4 * gfx::Vertex3D::STRIDE);
		let p1 = vec3!(0);
		let p2 = vec3!(0);
		let p3 = vec3!(0);
		let p4 = vec3!(0);

		gfx::Vertex3D {
			pos: p1,
			normal: self.normal,
			color: self.color,
			uv: vec2!(0),
		}.push(&mut verts);

		gfx::Vertex3D {
			pos: p2,
			normal: self.normal,
			color: self.color,
			uv: vec2!(0),
		}.push(&mut verts);

		gfx::Vertex3D {
			pos: p3,
			normal: self.normal,
			color: self.color,
			uv: vec2!(0),
		}.push(&mut verts);

		gfx::Vertex3D {
			pos: p4,
			normal: self.normal,
			color: self.color,
			uv: vec2!(0),
		}.push(&mut verts);

		ctx.renderer_3d.push(
			gl::Primitive::Triangle,
			&verts,
			&[0, 1, 2, 1, 2, 3],
			&ctx.cur_pipeline_3d,
			&gfx::Uniform3D {
				proj: ctx.proj,
				view: ctx.view,
				model: ctx.transform,
				color: rgba!(1),
				tex: ctx.empty_tex.clone(),
				custom: ctx.cur_custom_uniform_3d.clone(),
			},
		)?;

		return Ok(());

	}

}


