// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Rect {
	p1: Vec2,
	p2: Vec2,
	radius: Option<f32>,
	fill: Option<Color>,
	stroke: Option<Stroke>,
}

impl Rect {
	pub fn from_pts(p1: Vec2, p2: Vec2) -> Self {
		return Self {
			p1: p1,
			p2: p2,
			radius: None,
			stroke: None,
			fill: Some(rgba!(1)),
		};
	}
	pub fn from_size(o: gfx::Origin, w: f32, h: f32) -> Self {
		let pt = o.as_pt();
		let p1 = (-pt * 0.5 + vec2!(-0.5, -0.5)) * vec2!(w, h);
		let p2 = (-pt * 0.5 + vec2!(0.5, 0.5)) * vec2!(w, h);
		return Self::from_pts(p1, p2);
	}
	pub fn from_rect(r: geom::Rect) -> Self {
		return Self::from_pts(r.min, r.max);
	}
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = Some(r);
		return self
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
		return self
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
}

pub fn rect(p1: Vec2, p2: Vec2) -> Rect {
	return Rect::from_pts(p1, p2);
}

pub fn rect2(o: gfx::Origin, w: f32, h: f32) -> Rect {
	return Rect::from_size(o, w, h);
}

impl Drawable for Rect {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let pts = vec![
			self.p1,
			vec2!(self.p2.x, self.p1.y),
			self.p2,
			vec2!(self.p1.x, self.p2.y),
		];

		let poly = Polygon {
			pts: pts.to_vec(),
			fill: self.fill,
			stroke: self.stroke.clone(),
			radius: self.radius,
		};

		ctx.draw(&poly)?;

		return Ok(());

	}

}

