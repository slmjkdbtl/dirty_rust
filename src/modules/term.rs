// wengwengweng

use crate::math::Color;
use crate::Result;

pub struct Term {
	term: console::Term,
}

impl Term {

	pub fn new() -> Self {

		return Self {
			term: console::Term::stdout(),
		};
	}

	pub fn write_line(&self, l: &str) -> Result<()> {
		return Ok(self.term.write_line(l)?);
	}

	pub fn width(&self) -> u16 {
		return self.term.size().1;
	}

	pub fn height(&self) -> u16 {
		return self.term.size().0;
	}

	pub fn render_text<S: AsRef<str>>(&self, lines: &[S]) {
		// ...
	}

	pub fn render_color(&self, buffer: &[Color]) {
		// ...
	}

	pub fn read_line(&self) -> Result<String> {
		return Ok(self.term.read_line()?);
	}

	pub fn read_char(&self) -> Result<char> {
		return Ok(self.term.read_char()?);
	}

	pub fn clear(&self) {
		self.term.clear_screen();
	}

	pub fn clear_line(&self) {
		self.term.clear_line();
	}

}

