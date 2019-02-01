// wengwengweng

use super::*;
use crate::*;

// use utils;

const BUTTON_WIDTH: u32 = 120;
const BUTTON_HEIGHT: u32 = 64;

/// canvas for drawing custom stuff
pub struct Button {
	text: String,
}

impl Button {

	pub fn new(t: &str) -> Self {
		return Self {
			text: String::from(t),
		};
	}

}

impl Widget for Button {

	fn draw(&self) {
// 		g2d::poly(&utils::rounded_rect(BUTTON_WIDTH, BUTTON_HEIGHT, CORNER));
	}

}

