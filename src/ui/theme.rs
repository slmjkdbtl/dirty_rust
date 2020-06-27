// wengwengweng

use super::*;

/// UI Theme
#[derive(Clone, Copy, Debug)]
pub struct Theme {
	pub bar_color: Color,
	pub border_color: Color,
	pub line_width: f32,
	pub bg_color: Color,
	pub title_color: Color,
	pub padding: f32,
	pub font_size: f32,
}

impl Default for Theme {
	fn default() -> Self {
		return Self {
			bar_color: rgba!(0, 0.51, 0.51, 1.0),
			border_color: rgba!(0.02, 0.18, 0.18, 1.0),
			line_width: 2.0,
			bg_color: rgba!(0, 0.35, 0.35, 1.0),
			title_color: rgba!(0.6, 0.78, 0.78, 1.0),
			padding: 8.0,
			font_size: 11.0,
		};
	}
}

