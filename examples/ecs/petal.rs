// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

comp!(Petal {

	flower: Id,
	index: u8

});

impl Petal {

	pub fn new(flower: Id, index: u8) -> Self {

		return Self {

			flower: flower,
			index: index,

		};

	}

}

