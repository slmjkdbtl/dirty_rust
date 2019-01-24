// wengwengweng

use dirty::*;
use dirty::addons::res;
use dirty::addons::ecs::*;

mod trans;
mod vel;
mod sprite;
mod body;

use trans::*;
use vel::*;
use sprite::*;
use body::*;

fn main() {

	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

	app::set_debug(true);
	res::load_sprites("examples/assets/", &vec!["car"]);
	res::load_sounds("examples/assets/", &vec!["pop", "yo"]);

	let (width, height) = window::size();
	let mut scene = scene();

	scene.add(car(vec2!(rand!(width), rand!(height))));

	app::run(&mut || {
		render(&mut scene);
		anim(&mut scene);
		debug(&mut scene);
	});

}

fn debug(s: &mut Scene) {

	if !app::debug() {
		return;
	}

	for e in s.get_all(comp_filter![Body]) {

		let b = e.get::<Body>();

		gfx::push();
		gfx::line_width(2);
		gfx::color(color!(0, 1, 1, 1));
		gfx::poly(&b.d_verts);
		gfx::pop();

	}

}

fn anim(s: &mut Scene) {

	for e in s.get_all(comp_filter![Sprite, Trans]) {

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

	for e in s.get_all(comp_filter![Sprite, Trans]) {

		let t = e.get::<Trans>();
		let s = e.get::<Sprite>();

		gfx::push();
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

fn car(pos: Vec2) -> Entity {

	let trans = Trans::new(pos, 0.0, vec2!(1));
	let mut sprite = Sprite::new("car");
	let vel = Vel::new(vec2!());
	let body = Body::new(&sprite.get_verts());

	sprite.play("run");
	sprite.speed = 0.0;

	return entity![trans, sprite, vel, body];

}

