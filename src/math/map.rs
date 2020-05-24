// wengwengweng

use std::ops::*;

pub trait Mapping: Sized + Copy
	+ Add<Output=Self>
	+ Sub<Output=Self>
	+ Mul<Output=Self>
	+ Div<Output=Self>
{

	fn map(self, a1: Self, a2: Self, b1: Self, b2: Self) -> Self {
		let r: Self = (self - a1) / (a2 - a1);
		return b1 + (b2 - b1) * r;
	}

}

impl Mapping for f32 {}