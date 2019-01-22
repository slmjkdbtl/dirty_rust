// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

#[derive(Clone, Debug)]
pub struct Vel {
	pub vel: Vec2,
}

impl Comp for Vel {}

impl Vel {

	pub fn new() -> Self {
		return Self::default();
	}

	pub fn vel(self, vel: Vec2) -> Self {
		return Self {
			vel: vel,
			..self
		};
	}

}

impl Default for Vel {

	fn default() -> Self {
		return Self {
			vel: vec2!(),
		};
	}

}


