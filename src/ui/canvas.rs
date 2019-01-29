// wengwengweng

use super::*;
use crate::*;

/// canvas for drawing custom stuff
pub struct Canvas {
	handle: gfx::Canvas,
}

impl Canvas {

	pub fn new(w: u32, h: u32) -> Self {
		return Self {
			handle: gfx::Canvas::new(w, h),
		};
	}

	pub fn set<F: FnMut()>(&self, mut f: F) {

		gfx::drawon(&self.handle);
		f();
		gfx::stop_drawon(&self.handle);

	}

}

impl Widget for Canvas {

	fn draw(&self) {
		gfx::render(&self.handle);
	}

}

