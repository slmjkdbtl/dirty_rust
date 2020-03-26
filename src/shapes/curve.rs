// wengwengweng

use super::*;

fn cosine(t: f32, p1: Vec2, p2: Vec2) -> Vec2 {

	let t2 = (1.0 - f32::cos(t * PI)) / 2.0;
	let x = p1.x * (1.0 - t) + p2.x * t;
	let y = p1.y * (1.0 - t2) + p2.y * t2;

	return vec2!(x, y);

}

// TODO
#[derive(Clone)]
pub struct Curve<'a> {
	pts: &'a [Vec2],
	dt: f32,
}

impl<'a> Curve<'a> {
	pub fn new(pts: &'a [Vec2]) -> Self {
		return Self {
			pts: pts,
			dt: 0.01,
		};
	}
	pub fn dt(mut self, dt: f32) -> Self {
		self.dt = dt;
		return self;
	}
}

pub fn curve<'a>(pts: &'a [Vec2]) -> Curve<'a> {
	return Curve::new(pts);
}

pub fn curve_samples(pts: &[Vec2], dt: f32) -> Vec<Vec2> {

	let mut samples = Vec::with_capacity(f32::ceil(1.0 / dt) as usize);

	for i in 0..pts.len() - 1 {

		let p1 = pts[i];
		let p2 = pts[i + 1];
		let mut t = 0.0;

		while t <= 1.0 {
			samples.push(cosine(t, p1, p2));
			t += dt;
		}

	}

	return samples;

}

impl<'a> Drawable for Curve<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let samples = curve_samples(self.pts, self.dt);

		for i in 0..samples.len() - 1 {

			let p1 = samples[i];
			let p2 = samples[i + 1];

			ctx.draw(&line(p1, p2))?;

		}

		return Ok(());

	}

}

