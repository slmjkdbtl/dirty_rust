// wengwengweng

//! TUI Utilities

use std::env;

pub use console::style;

/// standard term colors
#[derive(Clone, Copy, Debug)]
pub enum Color {
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
}

/// standard term styles
#[derive(Clone, Copy, Debug)]
pub enum Style {
	Bold,
	Italic,
	Underline,
}

impl Color {

	pub fn fg_code(&self) -> &str {
		return match *self {
			Color::Black => "30",
			Color::Red => "31",
			Color::Green => "32",
			Color::Yellow => "33",
			Color::Blue => "34",
			Color::Magenta => "35",
			Color::Cyan => "36",
			Color::White => "37",
		};
	}

	pub fn bg_code(&self) -> &str {
		return match *self {
			Color::Black => "40",
			Color::Red => "41",
			Color::Green => "42",
			Color::Yellow => "43",
			Color::Blue => "44",
			Color::Magenta => "45",
			Color::Cyan => "46",
			Color::White => "47",
		};
	}

}

