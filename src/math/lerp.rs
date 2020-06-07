// wengwengweng

use std::f32::consts::PI;
use std::ops::*;

pub trait Lerp:
	Copy
	+ Add<Output=Self>
	+ Sub<Output=Self>
	+ Mul<f32, Output=Self>
{

	fn lerp(self, to: Self, t: f32) -> Self {
		return self + (to - self) * t.max(0.0).min(1.0);
	}

	fn cos_lerp(self, to: Self, t: f32) -> Self {
		let t2 = (1.0 - f32::cos(t.max(0.0).min(1.0) * PI)) / 2.0;
		return self.lerp(to, t2);
	}

}

impl<T> Lerp for T
	where T:
		Copy
		+ Add<Output=T>
		+ Sub<Output=T>
		+ Mul<f32, Output=T>
{}

