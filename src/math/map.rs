// wengwengweng

use std::ops::*;

// TODO: trait alias

pub trait Mappable:
	Copy
	+ Add<Output=Self>
	+ Sub<Output=Self>
	+ Mul<f32, Output=Self>
	+ Div<Output=f32>
{
	fn map(self, a1: Self, a2: Self, b1: Self, b2: Self) -> Self {
		let r = (self - a1) / (a2 - a1);
		return b1 + (b2 - b1) * r;
	}
}

impl<T> Mappable for T
	where T: Copy
		+ Add<Output=T>
		+ Sub<Output=T>
		+ Mul<f32, Output=Self>
		+ Div<Output=f32>
{}

