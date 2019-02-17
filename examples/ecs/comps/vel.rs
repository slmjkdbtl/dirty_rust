// wengwengweng

use sock::*;
use sock::math::*;
use sock::ecs::*;
use sock::ecs::derive::*;

#[derive(Comp, Clone)]
pub struct Vel {

	pub pos: Vec2,
	pub rot: f32,
	pub scale: Vec2,

}

impl Vel {

	pub fn new(pos: Vec2, rot: f32, scale: Vec2) -> Self {

		return Self {

			pos: pos,
			rot: rot,
			scale: scale,

		};

	}

}

impl Default for Vel {

	fn default() -> Self {

		return Self {

			pos: vec2!(),
			rot: 0.0,
			scale: vec2!(1),

		};

	}

}

