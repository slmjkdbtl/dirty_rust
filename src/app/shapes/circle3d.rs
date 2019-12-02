// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Circle3D {
	pt: Vec3,
	radius: f32,
	color: Color,
}

pub fn circle3d(p: Vec3, r: f32) -> Circle3D {
	return Circle3D::new(p, r);
}

impl Circle3D {
	pub fn new(p: Vec3, r: f32) -> Self {
		return Self {
			pt: p,
			radius: r,
			color: rgba!(),
		};
	}
	pub fn color(mut self, c: Color) -> Self {
		self.color = c;
		return self;
	}
}

impl Drawable for Circle3D {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let spt = ctx.to_sc(self.pt);

		ctx.draw(
			&circle(spt, self.radius)
				.fill(self.color)
		)?;

		return Ok(());

	}

}

