// wengwengweng

use super::*;

pub struct Slider<T> {
	pub(crate) val: T,
	pub(crate) min: T,
	pub(crate) max: T,
}

impl<T: 'static> Widget for Slider<T> {
}

