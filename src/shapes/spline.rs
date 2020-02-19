// wengwengweng

use super::*;

impl splines::interpolate::Linear<f32> for Vec2 {
	fn outer_mul(self, t: f32) -> Self {
		return self * t;
	}
	fn outer_div(self, t: f32) -> Self {
		return self / t;
	}
}

impl splines::Interpolate<f32> for Vec2 {
	fn lerp(a: Self, b: Self, t: f32) -> Self {
		return a * (1. - t) + b * t;
	}

	fn cubic_hermite(x: (Self, f32), a: (Self, f32), b: (Self, f32), y: (Self, f32), t: f32) -> Self {
		return splines::interpolate::cubic_hermite_def(x, a, b, y, t);
	}

	fn quadratic_bezier(a: Self, u: Self, b: Self, t: f32) -> Self {
		return splines::interpolate::quadratic_bezier_def(a, u, b, t);
	}

	fn cubic_bezier(a: Self, u: Self, v: Self, b: Self, t: f32) -> Self {
		return splines::interpolate::cubic_bezier_def(a, u, v, b, t);
	}
}

pub use splines::Interpolation as Interp;

// TODO
#[derive(Clone)]
pub struct Spline {
	dt: f32,
	spline: splines::Spline<f32, Vec2>,
}

impl Spline {

	pub fn from_pts(pts: &[(f32, Vec2)]) -> Self {

		use splines::Key;

		let keys = pts
			.iter()
			.map(|(t, p)| Key::new(*t, *p, Interp::Cosine))
			.collect();

		let spline = splines::Spline::from_vec(keys);

		return Self {
			dt: 0.1,
			spline: spline,
		};

	}

}

impl Drawable for Spline {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let mut step = 0.0;
		let mut samples = vec![];

		while step <= 1.0 {
			if let Some(sample) = self.spline.sample(step) {
				samples.push(sample);
			}
			step += self.dt;
		}

		for i in 0..samples.len() - 1 {
			ctx.draw(&line(samples[i], samples[i + 1]))?;
		}

		return Ok(());

	}

}

