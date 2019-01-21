// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

comp!(Trans {

	pos: Vec2 (vec2!()),
	rot: f32 (0.0),
	scale: Vec2 (vec2!(1)),

});

impl Trans {

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

