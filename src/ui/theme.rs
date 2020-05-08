// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
	pub bar_color: Color,
	pub border_color: Color,
	pub border_thickness: f32,
	pub background_color: Color,
	pub title_color: Color,
	pub padding: Vec2,
	pub margin: f32,
	pub font_size: f32,
}

impl Default for Theme {
	fn default() -> Self {
		return Self {
			bar_color: rgba!(0, 0.51, 0.51, 0.9),
			border_color: rgba!(0.02, 0.18, 0.18, 0.9),
			border_thickness: 2.0,
			background_color: rgba!(0, 0.35, 0.35, 0.9),
			title_color: rgba!(0.6, 0.78, 0.78, 0.9),
			padding: vec2!(9),
			margin: 9.0,
			font_size: 12.0,
		};
	}
}

