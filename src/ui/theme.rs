// wengwengweng

use crate::math::*;

/// define styles
#[derive(Clone, Copy, Debug)]
pub struct Theme {

	pub back: Color,
	pub line: Color,
	pub bar: Color,
	pub text_active: Color,
	pub text_passive: Color,

}

impl Default for Theme {
	fn default() -> Self {
		return Self {
			back: Color::from_hex(0x005A5A, 1.0),
			line: Color::from_hex(0x052D2D, 1.0),
			bar: Color::from_hex(0x008282, 1.0),
			text_active: Color::from_hex(0xFFFFFF, 1.0),
			text_passive: Color::from_hex(0x9AC7C7, 1.0),
		}
	}
}

