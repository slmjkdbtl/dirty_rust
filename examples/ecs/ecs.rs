// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

use std::collections::HashMap;

mod trans;
mod sprite;

use trans::*;
use sprite::*;

fn main() {

	let mut s = scene();

	s.add(car(vec2!(120)));
	s.add(car(vec2!(20)));

	for e in s.get(&comp_filter![Trans, Sprite]) {
		dbg!(e.get::<Trans>());
	}

}

fn car(pos: Vec2) -> Entity {

	let trans = Trans::default()
		.pos(pos);

	let sprite = Sprite::default();

	return entity![trans, sprite];

}

