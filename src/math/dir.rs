// wengwengweng

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
	X,
	Y,
	Z,
}

impl Axis {
	pub fn as_vec3(&self) -> Vec3 {
		return match self {
			Axis::X => vec3!(1, 0, 0),
			Axis::Y => vec3!(0, 1, 0),
			Axis::Z => vec3!(0, 0, 1),
		};
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
	Right,
	Down,
	Left,
	Up,
}

impl Dir {
	pub fn as_vec2(&self) -> Vec2 {
		return match self {
			Dir::Right => vec2!(1, 0),
			Dir::Down => vec2!(0, 1),
			Dir::Left => vec2!(-1, 0),
			Dir::Up => vec2!(0, -1),
		};
	}
}

