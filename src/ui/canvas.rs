// wengwengweng

use super::*;
use crate::*;

const BAR_HEIGHT: u32 = 42;

/// canvas for drawing custom stuff
pub struct Canvas {
	handle: gfx::Canvas,
}

impl Canvas {

	pub fn new(w: &Window) -> Self {
		return Self {
			handle: gfx::Canvas::new(w.width, w.height - BAR_HEIGHT),
		};
	}

}

impl Widget for Canvas {

	fn draw(&self) {
		gfx::render(&self.handle);
	}

	fn get_type(&self) -> WidgetType {
		return WidgetType::Exclusive;
	}

}

