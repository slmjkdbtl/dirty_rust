// wengwengweng

use dirty::*;
use specs::*;
use specs_derive::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Trans {

	pub pos: Vec2,
	pub rot: f32,
	pub scale: Vec2,

}

impl Trans {

	pub fn new() -> Self {
		return Self::default();
	}

	pub fn pos(self, pos: Vec2) -> Self {
		return Self {
			pos: pos,
			..self
		}
	}

	pub fn scale(self, scale: Vec2) -> Self {
		return Self {
			scale: scale,
			..self
		}
	}

	pub fn rot(self, rot: f32) -> Self {
		return Self {
			rot: rot,
			..self
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

