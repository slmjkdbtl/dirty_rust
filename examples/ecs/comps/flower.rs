// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::kit::*;

#[derive(Clone)]
pub enum Player {

	One,
	Two,

}

#[derive(Clone)]
pub struct Flower {

	pub player: Player,
	pub color: Color,
	pub speed: f32,
	pub rot_speed: f32,
	pub energy: usize,
	pub rate: usize,
	pub active: bool,

}

impl Flower {

	pub fn new(p: Player) -> Self {

		let color = match p {

			Player::One => color!(1, 1, 0, 1),
			Player::Two => color!(0, 1, 1, 1),

		};

		return Self {

			player: p,
			color: color,
			speed: 60.0,
			rot_speed: 3.0,
			energy: 0,
			rate: 16,
			active: false,

		};

	}

}

