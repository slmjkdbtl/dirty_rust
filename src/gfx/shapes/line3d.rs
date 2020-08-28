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
	return Line3D::new(p1, p2);
}

impl Line3D {
	pub fn new(p1: Vec3, p2: Vec3) -> Self {
		return Self {
			p1,
			p2,
			color: rgba!(1),
			width: 1.0,
		};
	}
	pub fn from_ray(r: geom::Ray3, d: f32) -> Self {
		return Self {
			p1: r.origin,
			p2: r.at(d),
			color: rgba!(),
			width: 1.0,
		};
	}
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
		return self;
	}
}

impl Drawable for Line3D {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		return ctx.draw(
			&raw(
				&[
					Vertex {
						pos: self.p1,
						normal: vec3!(1),
						color: self.color,
						uv: vec2!(0),
					},
					Vertex {
						pos: self.p2,
						normal: vec3!(1),
						color: self.color,
						uv: vec2!(0),
					},
				],
				&[0, 1]
			)
				.prim(Primitive::Line(self.width))
		);

	}

}

