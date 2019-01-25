// wengwengweng

use dirty::*;
use dirty::addons::res;
use specs::*;

mod trans;
mod vel;
mod sprite;
mod body;
mod flower;
mod move_sys;
mod render_sys;
mod anim_sys;
mod debug_sys;

use trans::*;
use vel::*;
use sprite::*;
use body::*;
use flower::*;
use move_sys::*;
use render_sys::*;
use anim_sys::*;
use debug_sys::*;

fn main() {

	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

	window::scale(window::Scale::X4);
	app::set_debug(true);
	res::load_sprites("examples/assets/", &vec!["core", "petal"]);

	let mut world = World::new();

    world.register::<Trans>();
    world.register::<Sprite>();
    world.register::<Body>();
    world.register::<Vel>();
    world.register::<Flower>();

	let mut dispatcher = DispatcherBuilder::new()
		.with(MoveSys, "move", &[])
		.with(AnimSys, "anim", &[])
		.with_thread_local(RenderSys)
		.with_thread_local(DebugSys)
		.build();

	let (width, height) = window::size();

	let f1 = create_flower(&mut world, Player::One, vec2!(rand!(width), rand!(height)));
	let f2 = create_flower(&mut world, Player::Two, vec2!(rand!(width), rand!(height)));

	app::run(&mut || {

		dispatcher.dispatch(&mut world.res);
		world.maintain();

	});

}

fn create_flower(world: &mut World, p: Player, pos: Vec2) -> Entity {

	let mut sprite = Sprite::new("core");
	let flower = Flower::new(p);

	sprite.color = flower.color;

	return world
		.create_entity()
		.with(Trans::new(pos, 0.0, vec2!(1)))
		.with(Vel::new(vec2!()))
		.with(flower)
		.with(Body::new(&sprite.get_verts()))
		.with(sprite)
		.build();

}

fn create_petal(world: &mut World, flower: Entity) -> Entity {

	let sprite = Sprite::new("petal");

	return world
		.create_entity()
		.with(Trans::default())
		.with(Vel::new(vec2!()))
		.with(Body::new(&sprite.get_verts()))
		.with(sprite)
		.build();

}

