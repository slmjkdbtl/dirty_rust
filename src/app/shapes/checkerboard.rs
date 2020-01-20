// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Checkerboard {
	size: f32,
	c1: Color,
	c2: Color,
	p1: Vec2,
	p2: Vec2,
}

impl Checkerboard {
	pub fn new(p1: Vec2, p2: Vec2, s: f32) -> Self {
		return Self {
			size: s,
			c1: rgba!(0.5, 0.5, 0.5, 1),
			c2: rgba!(0.75, 0.75, 0.75, 1),
			p1: p1,
			p2: p2,
		};
	}
	pub fn color(mut self, c1: Color, c2: Color) -> Self {
		self.c1 = c1;
		self.c2 = c2;
		return self;
	}
}

pub fn checkerboard(p1: Vec2, p2: Vec2, s: f32) -> Checkerboard {
	return Checkerboard::new(p1, p2, s);
}

impl gfx::Drawable for Checkerboard {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let p1 = vec2!(f32::min(self.p1.x, self.p2.x), f32::min(self.p1.y, self.p2.y));
		let p2 = vec2!(f32::max(self.p1.x, self.p2.x), f32::max(self.p1.y, self.p2.y));

		let mut p = p1;
		let mut g = true;
		let mut dir = 1.0;

		while p.y < p2.y {

			let x = f32::min(p.x + self.size, p2.x);
			let y = f32::min(p.y + self.size, p2.y);

			let color = if g {
				rgba!(0.5, 0.5, 0.5, 1)
			} else {
				rgba!(0.75, 0.75, 0.75, 1)
			};

			g = !g;

			ctx.draw(&rect(p, vec2!(x, y)).fill(color))?;

			let nx = p.x + self.size * dir;

			if nx >= p2.x || nx < p1.x {
				p.y += self.size;
				dir = -dir;
			} else {
				p.x = nx;
			}

		}

		return Ok(());

	}

}

