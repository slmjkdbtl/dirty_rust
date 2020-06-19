// wengwengweng

//! Utilities for Terminal Output

use std::fmt;
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
		let (r, g, b, a) = c.as_u8();
		self.text = ansi_wrap(&self.text, &format!("38;2;{};{};{}", r, g, b));
		return self;
	}
	pub fn bg_truec(mut self, c: Color) -> Self {
		let (r, g, b, a) = c.as_u8();
		self.text = ansi_wrap(&self.text, &format!("48;2;{};{};{}", r, g, b));
		return self;
	}
}

impl fmt::Display for StyledOutput {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		return write!(f, "{}", self.text);
	}
}

impl fmt::Debug for StyledOutput {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		return write!(f, "{}", self);
	}
}

pub fn style(s: &str) -> StyledOutput {
	return StyledOutput {
		text: String::from(s),
	};
}

