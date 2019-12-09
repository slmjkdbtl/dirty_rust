// wengwengweng

use std::ops::*;

pub trait Mappable =
	Copy
	+ Add<Self, Output = Self>
	+ Sub<Self, Output = Self>
	+ Mul<Self, Output = Self>
	+ Div<Self, Output = Self>
	;

pub trait Mapping<T: Mappable + Mul<Self, Output = T>>: Mappable {

	/// map a value to another range
	fn map(self, a1: Self, a2: Self, b1: T, b2: T) -> T {
		let r: Self = (self - a1) / (a2 - a1);
		return b1 + (b2 - b1) * r;
	}

}

impl<T: Mappable, F: Mappable + Mul<T, Output = F>> Mapping<F> for T {}

