// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

#[derive(Clone, Debug)]
pub struct Vel {
	vel: Vec2,
}

impl Comp for Vel {}

impl Vel {

	pub fn new() -> Self {
		return Self::default();
	}

}

impl Default for Vel {

	fn default() -> Self {
		return Self {
			vel: vec2!(),
		};
	}

}


