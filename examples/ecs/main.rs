// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;
use dirty::addons::res;

mod trans;
mod sprite;
mod body;
mod vel;
mod move_sys;
mod render_sys;
mod debug_sys;

use trans::*;
use sprite::*;
use body::*;
use vel::*;
use move_sys::*;
use render_sys::*;
use debug_sys::*;

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

	s.run(MoveSys);
	s.run(RenderSys);
	s.run(DebugSys);

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

