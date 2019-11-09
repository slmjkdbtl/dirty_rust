// wengwengweng

use std::ops::*;

pub trait Lerpable =
	Copy
	+ Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<f32, Output = Self>
	;

pub trait Lerping: Lerpable {

	/// linear interpolation
	fn lerp(self, to: Self, amount: f32) -> Self {
		return self + (to - self) * amount.clamp(0.0, 1.0);
	}

	/// cubic interpolation
	fn smooth(self, to: Self, amount: f32) -> Self {

		let t = amount.clamp(0.0, 1.0);
		let m = t * t * (3.0 - 2.0 * t);

		return self + (to - self) * m;

	}

}

impl<T: Lerpable> Lerping for T {}

