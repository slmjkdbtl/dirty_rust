// wengwengweng

use super::*;

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

	pub fn from_window(w: &Window) -> Self {
		return Self::new(w.width - 4, w.height - BAR_HEIGHT - 5);
	}

	pub fn set<F: FnMut()>(&self, mut f: F) {

		g2d::push();
		g2d::reset();
		gfx::drawon(&self.handle);
		f();
		gfx::stop_drawon(&self.handle);
		g2d::pop();

	}

}

impl Widget for Canvas {

	fn draw(&self) {
		g2d::color(color!(1));
		g2d::render(&self.handle);
	}

}

