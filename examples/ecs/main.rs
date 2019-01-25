// wengwengweng

use dirty::*;
use dirty::addons::res;
use dirty::addons::ecs::*;

mod trans;
mod vel;
mod sprite;
mod body;
mod powder;
mod flower;
mod petal;

use trans::*;
use vel::*;
use sprite::*;
use body::*;
use powder::*;
use flower::*;
use petal::*;

fn main() {

	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

	app::set_debug(true);
	res::load_sprites("examples/assets/", &vec!["core", "petal"]);
	res::load_sounds("examples/assets/", &vec!["pop", "yo"]);

	let (width, height) = window::size();
	let mut scene = scene();

	let one = scene.add(flower(Player::One, vec2!(rand!(width / 2), rand!(height / 2))));
	let two = scene.add(flower(Player::Two, vec2!(rand!(width / 2), rand!(height / 2))));

	for i in 0..4 {
		scene.add(petal(one, i));
		scene.add(petal(two, i));
	}

	app::run(&mut || {

		gfx::push();
		gfx::scale(vec2!(2));
		anim(&mut scene);
		petal_follow(&mut scene);
		render(&mut scene);
		gfx::pop();
// 		debug(&mut scene);

	});

}

fn debug(s: &mut Scene) {

	if !app::debug() {
		return;
	}

	for id in s.filter(comp_filter![Body]) {

		let e = s.get(id).unwrap();
		let b = e.get::<Body>();

		gfx::push();
		gfx::line_width(1);
		gfx::color(color!(1, 0, 1, 1));
		gfx::poly(&b.d_verts);
		gfx::pop();

	}

}

fn petal_follow(s: &mut Scene) {

	for id in s.filter(comp_filter![Petal, Trans]) {

		let e = s.get(id).unwrap();
		let mut petal = e.get::<Petal>();
		let mut trans = e.get::<Trans>();

		if let Some(flower) = s.get(petal.flower) {

			let f_trans = flower.get::<Trans>();
			let ang = petal.index as f32 * 90f32;

			trans.pos = f_trans.pos + Vec2::from_angle((ang - 90.0).to_radians()) * 1.2;
			trans.rot = f_trans.rot + ang.to_radians();
			s.get_mut(id).unwrap().set::<Trans>(trans);

		}

	}

}

fn anim(s: &mut Scene) {

	for id in s.filter(comp_filter![Sprite, Trans]) {

		let e = s.get_mut(id).unwrap();
		let mut s = e.get::<Sprite>();

		if let Some(anim) = s.current_anim {

			if s.timer >= s.speed {
				s.timer = 0.0;
				s.tick();
			} else {
				s.timer += app::dt();
			}

			e.set::<Sprite>(s);

		}

	}

}

fn render(s: &mut Scene) {

	for id in s.filter(comp_filter![Sprite, Trans]) {

		let e = s.get_mut(id).unwrap();
		let t = e.get::<Trans>();
		let s = e.get::<Sprite>();

		gfx::push();
		gfx::color(s.color);
		gfx::translate(t.pos);
		gfx::rotate(t.rot);
		gfx::translate(s.offset() * t.scale);
		gfx::scale(t.scale);

		if e.has::<Body>() {

			let mut body = e.get::<Body>();

			body.d_verts = gfx::multi_warp(&body.verts);
			e.set::<Body>(body);

		}

		gfx::draw(&s.tex, s.framelist[s.frame]);
		gfx::pop();

	}

}

fn flower(p: Player, pos: Vec2) -> Entity {

	let trans = Trans::new(pos, 0.0, vec2!(1));
	let mut sprite = Sprite::new("core");
	let vel = Vel::new(vec2!());
	let body = Body::new(&sprite.get_verts());
	let flower = Flower::new(p);

	sprite.color = flower.color;

	return entity![trans, sprite, vel, body, flower];

}

fn petal(flower: Id, index: u8) -> Entity {

	let trans = Trans::default();
	let mut sprite = Sprite::new("petal");
	let vel = Vel::new(vec2!());
	let body = Body::new(&sprite.get_verts());
	let petal = Petal::new(flower, index);

	sprite.origin = vec2!(0.5, 1);

	return entity![trans, sprite, vel, body, petal];

}

