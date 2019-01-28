// wengwengweng

use super::*;
use crate::*;

/// canvas for drawing custom stuff
pub struct TextBox {
	lines: Vec<String>,
}

impl TextBox {

	pub fn new() -> Self {
		return Self {
			lines: Vec::new(),
		};
	}

	pub fn write(&mut self, text: &str) {
		self.lines.push(String::from(text));
	}

}

impl Widget for TextBox {

	fn draw(&self) {

		gfx::push();

		for l in &self.lines {
			gfx::text(&l);
			gfx::translate(vec2!(0, 16));
		}

		gfx::pop();

	}

	fn get_type(&self) -> WidgetType {
		return WidgetType::Normal;
	}

}



