// wengwengweng

use dirty::*;
use dirty::math::*;
use specs::*;
use specs_derive::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {

	One,
	Two,

}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Flower {

	pub player: Player,
	pub color: Color,

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

		};

	}

}

