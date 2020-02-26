// wengwengweng

//! Terminal Utilities

use crate::math::Color;

#[derive(Clone)]
pub struct StyledOutput {
	text: String,
}

fn ansi_wrap(s: &str, code: &str) -> String {
	return format!("\x1b[{}m{}\x1b[0m", code, s);
}

macro_rules! ansi {
	({$($name:ident => $code:expr),*$(,)?}) => {
		impl StyledOutput {
			$(
				pub fn $name(mut self) -> Self {
					self.text = ansi_wrap(&self.text, $code);
					return self;
				}
			)*
		}
	}
}

ansi!({
	black => "30",
	red => "31",
	green => "32",
	yellow => "33",
	blue => "34",
	magenta => "35",
	cyan => "36",
	white => "37",
});

ansi!({
	bg_black => "40",
	bg_red => "41",
	bg_green => "42",
	bg_yellow => "44",
	bg_blue => "44",
	bg_magenta => "45",
	bg_cyan => "46",
	bg_white => "47",
});

ansi!({
	reset => "0",
	bold => "1",
	dim => "2",
	italic => "3",
	underline => "4",
	blink => "5",
});

impl StyledOutput {
	pub fn truec(mut self, c: Color) -> Self {
		let c = c.as_u8();
		self.text = ansi_wrap(&self.text, &format!("38;2;{};{};{}", c[0], c[1], c[2]));
		return self;
	}
	pub fn bg_truec(mut self, c: Color) -> Self {
		let c = c.as_u8();
		self.text = ansi_wrap(&self.text, &format!("48;2;{};{};{}", c[0], c[1], c[2]));
		return self;
	}
}

impl std::fmt::Display for StyledOutput {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		return write!(f, "{}", self.text);
	}
}

impl std::fmt::Debug for StyledOutput {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		return write!(f, "{}", self);
	}
}

pub fn style(s: &str) -> StyledOutput {
	return StyledOutput {
		text: String::from(s),
	};
}

pub fn display(pixels: &[Color], width: u32, height: u32) {

	let mut x = 0;
	let mut y = 0;
	let mut out = String::with_capacity(pixels.len());

	loop {

		let i1 = y * width as usize + x;
		let i2 = (y + 1) * width as usize + x;

		if let Some(c1) = pixels.get(i1) {
			if let Some(c2) = pixels.get(i2) {
				// draw upper block
				out.push_str(&format!("{}", style("\u{2580}").truec(*c1).bg_truec(*c2)))
			}
		}

		x += 1;

		if x >= width as usize {
			out.push('\n');
			x = 0;
			y += 2;
		}

		if y >= height as usize {
			break;
		}

	}

	print!("{}\x1b[{}A", out, height / 2);

}

