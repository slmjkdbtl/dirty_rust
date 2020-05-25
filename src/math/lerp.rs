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

	fn smooth(self, to: Self, amount: f32) -> Self {

		let t = amount.max(0.0).min(1.0);
		let m = t * t * (3.0 - 2.0 * t);

		return self + (to - self) * m;

	}

}

impl<T: Lerpable> Lerping for T {}

