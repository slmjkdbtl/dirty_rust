// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

pub struct Vel {
	pub pos: Vec2,
}

impl Vel {

	pub fn new(pos: Vec2) -> Self {
		return Self {
			pos: pos,
		};
	}

	pub fn pos(self, pos: Vec2) -> Self {
		return Self {
			pos: pos,
			..self
		};
	}

}

impl Default for Vel {

	fn default() -> Self {
		return Self {
			pos: vec2!(),
		};
	}

}

impl Comp for Vel {}

