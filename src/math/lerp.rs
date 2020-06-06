// wengwengweng

use std::ops::*;

// TODO: trait alias

pub trait Lerpable:
	Copy
	+ Add<Output=Self>
	+ Sub<Output=Self>
	+ Mul<f32, Output=Self>
{
	fn lerp(self, to: Self, amount: f32) -> Self {
		return self + (to - self) * amount.max(0.0).min(1.0);
	}
}

impl<T> Lerpable for T
	where T: Copy
		+ Add<Output=T>
		+ Sub<Output=T>
		+ Mul<f32, Output=T>
{}

