// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Circle {
	center: Vec2,
	radius: f32,
	segments: u32,
	stroke: Option<Stroke>,
	fill: Option<Color>,
}

impl Circle {
	pub fn new(center: Vec2, radius: f32) -> Self {
		return Self {
			center,
			radius,
			segments: (radius.sqrt() * 6.0) as u32,
			stroke: None,
			fill: Some(rgba!(1)),
		};
	}
	pub fn fill(mut self, c: Color) -> Self {
		self.fill = Some(c);
		return self;
	}
	pub fn no_fill(mut self) -> Self {
		self.fill = None;
		return self;
	}
	pub fn opacity(mut self, a: f32) -> Self {
		if let Some(fill) = &mut self.fill {
			fill.a = a;
		}
		if let Some(stroke) = &mut self.stroke {
			stroke.color.a = a;
		}
		return self;
	}
	pub fn stroke(mut self, c: Color) -> Self {
		self.stroke = Some(Stroke {
			width: 1.0,
			join: LineJoin::None,
			dash: None,
			color: c,
		});
		return self;
	}
	pub fn line_join(mut self, j: LineJoin) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.join = j;
		}
		return self;
	}
	pub fn line_width(mut self, w: f32) -> Self {
		if let Some(stroke) = &mut self.stroke {
			stroke.width = w;
		}
		return self;
	}
	pub fn segments(mut self, s: u32) -> Self {
		self.segments = s;
		return self
	}
}

pub fn circle(center: Vec2, radius: f32) -> Circle {
	return Circle::new(center, radius);
}

impl Drawable for Circle {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		if self.radius <= 0.0 {
			return Ok(());
		}

		let step = PI * 2.0 / self.segments as f32;

		let pts = (0..self.segments)
			.map(|i| Vec2::from_angle(i as f32 * step) * self.radius)
			.collect();

		let poly = Polygon {
			pts: pts,
			fill: self.fill,
			stroke: self.stroke.clone(),
		};

		ctx.draw_t(
			mat4!()
				.t2(self.center)
				,
			&poly
		)?;

		return Ok(());

	}

}


