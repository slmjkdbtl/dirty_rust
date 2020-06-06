// wengwengweng

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum PointMode {
	Rect,
	Circle,
}

#[derive(Clone)]
pub struct Points<'a> {
	pts: &'a[Vec2],
	size: f32,
	mode: PointMode,
	color: Color,
}

impl<'a> Points<'a> {
	pub fn from(pts: &'a[Vec2]) -> Self {
		return Self {
			pts,
			size: 1.0,
			color: rgba!(1),
			mode: PointMode::Rect,
		};
	}
	pub fn size(mut self, s: f32) -> Self {
		self.size = s;
		return self;
	}
	pub fn color(mut self, color: Color) -> Self {
		self.color = color;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		self.color.a = a;
		return self;
	}
	pub fn mode(mut self, m: PointMode) -> Self {
		self.mode = m;
		return self;
	}
}

pub fn points<'a>(pts: &'a[Vec2]) -> Points<'a> {
	return Points::from(pts);
}

impl<'a> Drawable for Points<'a> {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		for pt in self.pts {
			match self.mode {
				PointMode::Circle => {
					ctx.draw(
						&circle(*pt, self.size)
							.fill(self.color)
					)?;
				},
				PointMode::Rect => {
					ctx.draw(
						&rect(*pt - vec2!(self.size) * 0.5, *pt + vec2!(self.size) * 0.5)
							.fill(self.color)
					)?;
				},
			}
		}

		return Ok(());

	}

}

