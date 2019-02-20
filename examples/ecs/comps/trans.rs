// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::kit::*;

#[derive(Clone)]
pub struct Trans {

	pub pos: Vec2,
	pub rot: f32,
	pub scale: Vec2,

}

impl Trans {

	pub fn new(pos: Vec2, rot: f32, scale: Vec2) -> Self {
		return Self {
			pos: pos,
			rot: rot,
			scale: scale,
		}
	}

}

impl Default for Trans {

	fn default() -> Self {
		return Self {
			pos: vec2!(),
			rot: 0.0,
			scale: vec2!(1),
		};
	}

}

