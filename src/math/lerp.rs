// wengwengweng

use std::ops::*;

pub trait Lerpable:
	Copy
	+ Add<Output=Self>
	+ Sub<Output=Self>
	+ Mul<f32, Output=Self>
	where Self: Sized
{}

impl<T> Lerpable for T
	where T: Copy
		+ Add<Output=T>
		+ Sub<Output=T>
		+ Mul<f32, Output=T>
{}

pub trait Lerping: Lerpable {
	fn lerp(self, to: Self, amount: f32) -> Self {
		return self + (to - self) * amount.max(0.0).min(1.0);
	}
}

impl<T: Lerpable> Lerping for T {}

