// wengwengweng

use dirty::*;
use dirty::math::*;
use specs::*;
use specs_derive::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Index {
	One,
	Two,
	Three,
	Four,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Petal {

	pub flower: Entity,
	pub index: Index

}

impl Petal {

	pub fn new(flower: Entity, index: Index) -> Self {

		return Self {

			flower: flower,
			index: index,

		};

	}

	pub fn index_as_num(&self) -> u8 {

		return match self.index {
			Index::One => 0,
			Index::Two => 1,
			Index::Three => 2,
			Index::Four => 3,
		};

	}

}
