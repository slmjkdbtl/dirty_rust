// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

#[derive(Clone)]
pub enum Player {

	One,
	Two,

}

comp!(Flower {

	player: Player,
	color: Color,

});

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

