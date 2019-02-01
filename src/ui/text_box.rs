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

		let theme = ctx_get().theme;

		g2d::push();
		g2d::translate(vec2!(14, 12));
		g2d::color(theme.text_passive);

		for l in &self.lines {
			g2d::text(&l);
			g2d::translate(vec2!(0, 16));
		}

		g2d::pop();

	}

}

