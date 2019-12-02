// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Checkerboard {
	size: f32,
	c1: Color,
	c2: Color,
	w: f32,
	h: f32,
}

impl Checkerboard {
	pub fn new(w: f32, h: f32, s: f32) -> Self {
		return Self {
			size: s,
			c1: rgba!(0.5, 0.5, 0.5, 1),
			c2: rgba!(0.75, 0.75, 0.75, 1),
			w: w,
			h: h,
		};
	}
	pub fn color(mut self, c1: Color, c2: Color) -> Self {
		self.c1 = c1;
		self.c2 = c2;
		return self;
	}
}

pub fn checkerboard(w: f32, h: f32, s: f32) -> Checkerboard {
	return Checkerboard::new(w, h, s);
}

impl gfx::Drawable for Checkerboard {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let col = f32::ceil(self.w / self.size) as i32;
		let row = f32::ceil(self.h / self.size) as i32;

		for i in 0..col {

			for j in 0..row {

				let c = if (i + j) % 2 == 0 {
					rgba!(0.5, 0.5, 0.5, 1)
				} else {
					rgba!(0.75, 0.75, 0.75, 1)
				};

				ctx.draw(
					&rect(
						vec2!(i as f32 * self.size, j as f32 * self.size),
						vec2!((i + 1) as f32 * self.size, (j + 1) as f32 * self.size),
					)
						.fill(c)
				)?;

			}

		}

		return Ok(());

	}

}

