// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

use std::collections::HashMap;

mod trans;
mod sprite;
mod body;
mod vel;
mod move_system;
mod render_system;

use trans::*;
use sprite::*;
use body::*;
use vel::*;
use move_system::*;
use render_system::*;

fn main() {

	let mut s = scene();

	let a = car(vec2!(123));

	s.add(a);

	s.run(MoveSystem);
	s.run(RenderSystem);

	s.update();
	s.update();
	s.update();

}

fn car(pos: Vec2) -> Entity {

	let trans = Trans::new()
		.pos(pos);

	let vel = Vel::new()
		.vel(vec2!(2, 2));

	let sprite = Sprite::new(gfx::Texture::from_bytes(&fs::read_bytes("examples/assets/car.png")));

	return entity![trans, vel, sprite];

}

