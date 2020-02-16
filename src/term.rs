// wengwengweng

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

pub fn clear_line() -> String {
	return format!("\r\x1b[2K");
}

pub fn clear_screen() -> String {
	return format!("\r\x1b[2J\r\x1b[H");
}

pub fn move_cursor_down() -> String {
	return format!("\x1b[1B");
}

pub fn move_cursor_up() -> String {
	return format!("\x1b[1A");
}

pub fn move_cursor_to(x: usize, y: usize) -> String {
	return format!("\x1B[{};{}H", y + 1, x + 1);
}

pub fn upper_block() -> StyledOutput {
	return style("\u{2580}");
}

pub fn lower_block() -> StyledOutput {
	return style("\u{2584}");
}

// TODO
pub fn display(pixels: &[Color], width: i32, height: i32) {
	// ...
}

// pub fn show_cursor() -> String {
// 	return format!("\u{001B}[?25h");
// }

// pub fn hide_cursor() -> String {
// 	return format!("\u{001B}[?25l");
// }

