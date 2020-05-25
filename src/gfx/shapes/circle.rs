// wengwengweng

use super::*;

#[derive(Clone)]
pub struct Circle {
	center: Vec2,
	radius: f32,
	segments: Option<u32>,
	stroke: Option<Stroke>,
	fill: Option<Color>,
	range: (f32, f32),
}

impl Circle {
	pub fn new(center: Vec2, radius: f32) -> Self {
		return Self {
			center,
			radius,
			segments: None,
			stroke: None,
			fill: Some(rgba!(1)),
			range: (0.0, 2.0 * PI),
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
		self.segments = Some(s);
		return self
	}
	pub fn range(mut self, p1: f32, p2: f32) -> Self {
		self.range = (p1, p2);
		return self
	}
}

pub fn circle(center: Vec2, radius: f32) -> Circle {
	return Circle::new(center, radius);
}

// TODO: is this correct?
fn circle_segments(radius: f32) -> u32 {
	return (radius.sqrt() * 6.0) as u32;
}

fn normalize_angle(angle: f32) -> f32 {
	if angle < 0.0 {
		return PI * 2.0 + angle;
	} else {
		return angle;
	}
}

pub(super) fn rounded_poly_verts(verts: &[Vec2], radius: f32, segments: Option<u32>) -> Vec<Vec2> {

	let segments = segments.unwrap_or(circle_segments(radius));
	let segments = segments as usize;
	let mut nv = Vec::with_capacity(segments);
	let len = verts.len();

	for i in 0..len {

		// TODO: subtraction overflow
		let prev = verts.get(i - 1).copied().unwrap_or(verts[len - 1]);
		let p = verts[i];
		let next = verts.get(i + 1).copied().unwrap_or(verts[0]);
		let angle = normalize_angle(p.angle(prev) - p.angle(next));
		let dis = radius / f32::tan(angle / 2.0);

		let p1 = p + (prev - p) * (dis / (prev - p).len());
		let p2 = p + (next - p) * (dis / (next - p).len());

		let center = p + (p1 - p) + (p2 - p);

		let start_angle = center.angle(p1);
		let end_angle = start_angle + angle;

		let arc = arc_verts(radius, start_angle, end_angle, None)
			.iter()
			.map(|p| *p + center)
			.collect::<Vec<Vec2>>()
			;

		nv.extend_from_slice(&arc);

	}

	return nv;

}

pub(super) fn arc_verts(radius: f32, start: f32, end: f32, segments: Option<u32>) -> Vec<Vec2> {

	let (start, end) = if end < start {
		(end, start)
	} else {
		(start, end)
	};

	let segments = segments.unwrap_or(f32::ceil(circle_segments(radius) as f32 * (end - start) / (PI * 2.0)) as u32);
	let segments = segments as usize;
	let step = (end - start) / segments as f32;
	let mut verts = Vec::with_capacity(segments);

	for i in 0..=segments {

		let angle = start + i as f32 * step;
		verts.push(Vec2::from_angle(angle) * radius);

	}

	return verts;

}

impl Drawable for Circle {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		if self.radius < 0.0 {
			return Ok(());
		}

		let p1 = self.range.0.max(0.0).min(PI * 2.0);
		let p2 = self.range.1.max(0.0).min(PI * 2.0);

		let mut pts = arc_verts(self.radius, p1, p2, self.segments);

		if p1 != 0.0 || p2 != PI * 2.0 {
			pts.push(self.center);
		}

		let poly = Polygon {
			pts,
			fill: self.fill,
			stroke: self.stroke.clone(),
			radius: None,
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


