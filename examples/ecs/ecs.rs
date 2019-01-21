// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

use std::collections::HashMap;

mod trans;
mod sprite;
mod body;

use trans::*;
use sprite::*;

fn main() {

	let mut s = scene();

	s.add(car(vec2!(120)));
	s.add(car(vec2!(20)));

	for e in s.get(&comp_filter![Trans, Sprite]) {
		// ...
	}

}

fn car(pos: Vec2) -> Entity {

	let trans = Trans::new()
		.pos(pos);

// 	let sprite = Sprite::default();

	return entity![trans];

}

