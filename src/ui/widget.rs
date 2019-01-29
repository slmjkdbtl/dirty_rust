// wengwengweng

use std::any::Any;

/// components inside a window
pub trait Widget: Any {

	fn update(&mut self) {}
	fn draw(&self) {}

}

