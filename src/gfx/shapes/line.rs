// wengwengweng

use super::*;

#[derive(Clone)]
enum LineMode<'a> {
	Single(Vec2, Vec2),
	Multiple(&'a [Vec2]),
}

#[derive(Clone)]
pub struct Line<'a> {
	pts: LineMode<'a>,
	width: f32,
	color: Color,
	cap: LineCap,
	dash: Option<LineDash>,
}

impl<'a> Line<'a> {
	pub fn new(p1: Vec2, p2: Vec2) -> Self {
		return Self {
			pts: LineMode::Single(p1, p2),
			width: 1.0,
			color: rgba!(1),
			cap: LineCap::Butt,
			dash: None,
		};
	}
	pub fn multiple(pts: &'a [Vec2]) -> Self {
		return Self {
			pts: LineMode::Multiple(pts),
			width: 1.0,
			color: rgba!(1),
			cap: LineCap::Butt,
			dash: None,
		};
	}
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
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
	pub fn cap(mut self, c: LineCap) -> Self {
		self.cap = c;
		return self;
	}
	pub fn dashed(mut self, len: f32, interval: f32) -> Self {
		self.dash = Some(LineDash {
			len,
			interval,
		});
		return self;
	}
}

pub fn line<'a>(p1: Vec2, p2: Vec2) -> Line<'a> {
	return Line::new(p1, p2);
}

pub fn lines<'a>(pts: &'a [Vec2]) -> Line<'a> {
	return Line::multiple(pts);
}

impl<'a> Drawable for Line<'a> {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		match self.pts {

			LineMode::Single(p1, p2) => {

				if let Some(dash) = self.dash {

					let diff = p2 - p1;
					let nd = diff.unit();
					let len = diff.len();
					let mut l = 0.0;
					let mut nxt_p1 = p1;

					loop {

						let cp1 = nxt_p1;
						let mut cp2 = nxt_p1 + nd * dash.len;

						l += dash.len;

						if l >= len {
							cp2 = p2;
						}

						ctx.draw(&Line {
							pts: LineMode::Single(cp1, cp2),
							width: self.width,
							color: self.color,
							cap: self.cap,
							dash: None,
						})?;

						nxt_p1 = cp2 + nd * dash.interval;
						l += dash.interval;

						if l >= len {
							break;
						}

					}

				} else {

					let dpos1 = Vec2::normal(p2 - p1).unit() * self.width / 2.0;
					let dpos2 = Vec2::normal(p1 - p2).unit() * self.width / 2.0;
					let cp1 = p1 - dpos1;
					let cp2 = p1 + dpos1;
					let cp3 = p2 - dpos2;
					let cp4 = p2 + dpos2;

					ctx.draw(
						&polygon(&[cp1, cp2, cp3, cp4])
							.fill(self.color)
					)?;

					if let LineCap::Round = self.cap {
						ctx.draw(
							&circle(p1, self.width / 2.0)
								.fill(self.color)
						)?;
						ctx.draw(
							&circle(p2, self.width / 2.0)
								.fill(self.color)
						)?;
					}

				}

			},

			LineMode::Multiple(pts) => {
				for (p1, p2) in pts.iter().zip(pts.iter().skip(1)) {
					ctx.draw(&Line {
						pts: LineMode::Single(*p1, *p2),
						width: self.width,
						color: self.color,
						cap: self.cap,
						dash: self.dash,
					})?;
				}
			},

		}

		return Ok(());

	}

}

