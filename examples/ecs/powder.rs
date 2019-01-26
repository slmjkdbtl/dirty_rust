// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

comp!(Powder {

	flower: Id,
	speed: f32,
	dir: f32,

});

impl Powder {

	pub fn new(flower: Id, dir: f32) -> Self {

		return Self {

			flower: flower,
			speed: 12.0,
			dir: dir,

		};

	}

}

