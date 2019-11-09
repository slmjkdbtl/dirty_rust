// wengwengweng

use std::ops::*;

pub trait Mappable =
	Copy
	+ Add<Self, Output = Self>
	+ Sub<Self, Output = Self>
	+ Mul<Self, Output = Self>
	+ Div<Self, Output = Self>
	;

pub trait Mapping: Mappable {

	/// map a value to another range
	fn map(self, a1: Self, a2: Self, b1: Self, b2: Self) -> Self {
		return b1 + (self - a1) / (a2 - a1) * (b2 - b1);
	}

}

impl<T: Mappable> Mapping for T {}

