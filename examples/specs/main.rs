// wengwengweng

use dirty::*;
use dirty::addons::res;
use specs::*;

mod trans;
mod vel;
mod sprite;
mod body;
mod flower;
mod petal;
mod move_sys;
mod render_sys;
mod anim_sys;
mod petal_follow_sys;
mod debug_sys;

use trans::*;
use vel::*;
use sprite::*;
use body::*;
use flower::*;
use petal::*;
use move_sys::*;
use render_sys::*;
use anim_sys::*;
use petal_follow_sys::*;
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
    world.register::<Petal>();

	let mut dispatcher = DispatcherBuilder::new()
		.with(MoveSys, "move", &[])
		.with(AnimSys, "anim", &[])
		.with(PetalFollowSys, "petal_follow", &[])
		.with_thread_local(RenderSys)
// 		.with_thread_local(DebugSys)
		.build();

	let (width, height) = window::size();

	let rand_in_view = |margin| {
		return vec2!(rand!(margin, width - margin), rand!(margin, height - margin));
	};

	let f1 = create_flower(&mut world, Player::One, rand_in_view(24));
	let f2 = create_flower(&mut world, Player::Two, rand_in_view(24));

	let indices = [
		Index::One,
		Index::Two,
		Index::Three,
		Index::Four,
	];

	for i in 0..4 {
		create_petal(&mut world, f1, indices[i]);
		create_petal(&mut world, f2, indices[i]);
	}

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

fn create_petal(world: &mut World, flower: Entity, index: Index) -> Entity {

	let mut sprite = Sprite::new("petal");

	sprite.origin = vec2!(0.5, 1);

	return world
		.create_entity()
		.with(Trans::default())
		.with(Vel::new(vec2!()))
		.with(Body::new(&sprite.get_verts()))
		.with(Petal::new(flower, index))
		.with(sprite)
		.build();

}

