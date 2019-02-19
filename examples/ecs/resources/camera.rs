// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::ecs::*;

pub struct Camera {
	pub pos: Vec2,
	pub scale: Vec2,
}

impl Camera {
	pub fn new() -> Self {
		return Self {
			pos: vec2!(),
			scale: vec2!(1),
		}
	}
}

