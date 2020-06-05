// wengwengweng

use super::*;

pub type GradientStop = (f32, Color);

#[derive(Clone)]
pub struct Gradient {
	p1: Vec2,
	p2: Vec2,
	stops: Vec<GradientStop>,
	width: f32,
}

impl Gradient {
	pub fn from(p1: Vec2, p2: Vec2, stops: &[GradientStop]) -> Gradient {
		return Self {
			p1,
			p2,
			stops: stops.to_vec(),
			width: 1.0,
		};
	}
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
		return self;
	}
}

pub fn gradient(p1: Vec2, p2: Vec2, stops: &[GradientStop]) -> Gradient {
	return Gradient::from(p1, p2, stops);
}

impl Drawable for Gradient {

	fn draw(&self, ctx: &mut Gfx) -> Result<()> {

		if self.stops.len() < 2 {
			return Err(format!("need at least 2 points to draw a gradient"));
		}

		use gfx::Vertex;

		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);
		let mut verts = Vec::with_capacity(4 + 2 * (self.stops.len() - 2));

		let matrix = ctx.transform
			.t2((self.p1 + self.p2) * 0.5)
			.r(rot - 90f32.to_radians())
			;

		let w = self.width;
		let h = Vec2::dist(self.p1, self.p2);

		let mut last_pos = None;

		for stop in &self.stops {

			let (t, c) = *stop;

			if (last_pos.is_none()) && (t != 0.0) {
				return Err(format!("gradient step should start at 0.0"));
			}

			last_pos = Some(t);

			verts.push(Vertex {
				pos: matrix * vec3!(-w / 2.0, -h / 2.0 + h * t, 0.0),
				uv: vec2!(0),
				normal: vec3!(0, 0, 1),
				color: c
			});

			verts.push(Vertex {
				pos: matrix * vec3!(w / 2.0, -h / 2.0 + h * t, 0.0),
				uv: vec2!(0),
				normal: vec3!(0, 0, 1),
				color: c,
			});

		}

		if (last_pos != Some(1.0)) {
			return Err(format!("gradient step should end at 1.0"));
		}

		let indices = [
			0, 1, 2,
			1, 2, 3,
		];

		let indices: Vec<u32> = indices
			.iter()
			.cycle()
			.take((self.stops.len() - 1) * indices.len())
			.enumerate()
			.map(|(i, vertex)| vertex + i as u32 / 6 * 2 )
			.collect();

		ctx.draw(&raw(&verts, &indices).transformed())?;

		return Ok(());

	}

}

