// wengwengweng

use std::any::Any;

use crate::*;
use crate::math::*;

/// components inside a window
pub trait Widget: Any {

	fn update(&mut self) {}
	fn draw(&self) {}

}

