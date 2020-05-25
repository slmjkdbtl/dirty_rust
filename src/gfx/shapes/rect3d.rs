// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Rect3D {
	p1: Vec3,
	p2: Vec3,
	color: Color,
	line_width: f32,
}

pub fn rect3d(p1: Vec3, p2: Vec3) -> Rect3D {
	return Rect3D::from_pts(p1, p2);
}

impl Rect3D {
	pub fn from_pts(p1: Vec3, p2: Vec3) -> Self {
		return Self {
			p1,
			p2,
			color: rgba!(),
			line_width: 1.0,
		};
	}
	pub fn from_bbox(b: geom::BBox) -> Self {
		return Self::from_pts(b.min, b.max)
	}
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		self.line_width = w;
		return self;
	}
}

impl Drawable for Rect3D {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		let p1 = vec3!(self.p1.x, self.p2.y, self.p1.z);
		let p2 = vec3!(self.p2.x, self.p2.y, self.p1.z);
		let p3 = vec3!(self.p2.x, self.p1.y, self.p1.z);
		let p4 = self.p1;

		let p5 = vec3!(self.p1.x, self.p2.y, self.p2.z);
		let p6 = self.p2;
		let p7 = vec3!(self.p2.x, self.p1.y, self.p2.z);
		let p8 = vec3!(self.p1.x, self.p1.y, self.p2.z);

		ctx.draw(&line3d(p1, p2).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p2, p3).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p3, p4).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p4, p1).color(self.color).width(self.line_width))?;

		ctx.draw(&line3d(p5, p6).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p6, p7).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p7, p8).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p8, p5).color(self.color).width(self.line_width))?;

		ctx.draw(&line3d(p1, p5).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p2, p6).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p3, p7).color(self.color).width(self.line_width))?;
		ctx.draw(&line3d(p4, p8).color(self.color).width(self.line_width))?;

		return Ok(());

	}

}

