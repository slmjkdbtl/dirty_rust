// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;
use dirty::addons::res;

use std::collections::HashMap;

mod trans;
mod sprite;
mod body;
mod vel;
mod move_system;
mod render_system;
mod debug_system;

use trans::*;
use sprite::*;
use body::*;
use vel::*;
use move_system::*;
use render_system::*;
use debug_system::*;

fn main() {

	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

	res::load_sprites("examples/assets/", &vec!["car"]);
	res::load_sounds("examples/assets/", &vec!["pop", "yo"]);

	let (width, height) = window::size();
	let mut s = scene();

	for _ in 0..2 {
		s.add(car(vec2!(rand!(width), rand!(height))));
	}

	s.run(MoveSystem);
	s.run(RenderSystem);
	s.run(DebugSystem);

	app::run(&mut || {
		s.update();
	});

}

fn car(pos: Vec2) -> Entity {

	let trans = Trans::new()
		.pos(pos);

	let vel = Vel::new();
	let sprite = Sprite::new(res::sprite("car").tex.clone());
	let body = Body::from_verts(&sprite.get_verts());

	return entity![trans, vel, sprite, body];

}

