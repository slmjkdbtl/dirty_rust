// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::ecs::*;
use dirty::addons::res;

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

// 	app::set_debug(true);
	window::scale(window::Scale::X4);
	res::load_sprites("examples/assets/", &vec!["core", "petal", "pixel"]);
	res::load_sounds("examples/assets/", &vec!["pop", "yo"]);

	let mut world = World::new();

	create_flower(&mut world, Player::One);
	create_flower(&mut world, Player::Two);

	app::run(&mut || {

		anim(&mut world);
		flower_input(&mut world);
		shoot(&mut world);
		transform(&mut world);
		powder_update(&mut world);
		petal_follow(&mut world);
		render(&mut world);
		debug(&mut world);

	});

}

fn powder_update(s: &mut World) {

	for id in s.filter(comp_filter![Powder, Sprite, Trans]) {

		let e = s.get(id).unwrap();
		let p = e.get::<Powder>();

		if let Some(flower) = s.get(p.flower) {

			let f = flower.get::<Flower>();
			let fv = flower.get::<Vel>();

			if f.active {

				let e = s.get_mut(id).unwrap();
				let mut sprite = e.get::<Sprite>();
				let mut t = e.get::<Trans>();

				sprite.color = color!(rand!(2) as i32, rand!(2) as i32, rand!(2) as i32, 1);
				t.pos = t.pos + Vec2::from_angle(p.dir) * p.speed * app::dt() + fv.pos * app::dt();

				e.set::<Sprite>(sprite);
				e.set::<Trans>(t);

			}

		}

	}

}

fn transform(s: &mut World) {

	for id in s.filter(comp_filter![Trans, Vel]) {

		let e = s.get_mut(id).unwrap();
		let mut t = e.get::<Trans>();
		let v = e.get::<Vel>();

		t.pos = t.pos + v.pos * app::dt();
		t.rot = t.rot + v.rot * app::dt();
		e.set::<Trans>(t);

	}

}

fn shoot(s: &mut World) {

	let mut queue = Vec::new();

	for id in s.filter(comp_filter![Flower, Trans]) {

		let e = s.get_mut(id).unwrap();
		let mut f = e.get::<Flower>();
		let t = e.get::<Trans>();

		if f.energy >= f.rate {
			queue.push(powder(id, t.pos + Vec2::from_angle(t.rot) * 8, t.rot));
			f.energy = 0;
		}

		e.set::<Flower>(f);

	}

	for p in queue {
		s.add(p);
	}

}

fn flower_input(s: &mut World) {

	for id in s.filter(comp_filter![Flower, Vel]) {

		let e = s.get_mut(id).unwrap();
		let mut f = e.get::<Flower>();
		let mut v = e.get::<Vel>();

		match f.player {

			Player::One => {

				if window::key_down(Key::W) {
					v.pos = vec2!(0, -1) * f.speed;
					f.active = true;
				} else if window::key_down(Key::S) {
					v.pos = vec2!(0, 1) * f.speed;
					f.active = true;
				} else if window::key_down(Key::A) {
					v.pos = vec2!(-1, 0) * f.speed;
					f.active = true;
				} else if window::key_down(Key::D) {
					v.pos = vec2!(1, 0) * f.speed;
					f.active = true;
				} else {
					v.pos = vec2!(0);
					f.active = false;
				}

				if window::key_down(Key::Q) {
					v.rot = -1.0 * f.rot_speed;
					f.active = true;
					f.energy += 1;
				} else if window::key_down(Key::E) {
					v.rot = 1.0 * f.rot_speed;
					f.active = true;
					f.energy += 1;
				} else {
					v.rot = 0.0;
				}

			}

			Player::Two => {

				if window::key_down(Key::I) {
					v.pos = vec2!(0, -1) * f.speed;
					f.active = true;
				} else if window::key_down(Key::K) {
					v.pos = vec2!(0, 1) * f.speed;
					f.active = true;
				} else if window::key_down(Key::J) {
					v.pos = vec2!(-1, 0) * f.speed;
					f.active = true;
				} else if window::key_down(Key::L) {
					v.pos = vec2!(1, 0) * f.speed;
					f.active = true;
				} else {
					v.pos = vec2!(0);
					f.active = false;
				}

				if window::key_down(Key::U) {
					v.rot = -1.0 * f.rot_speed;
					f.active = true;
					f.energy += 1;
				} else if window::key_down(Key::O) {
					v.rot = 1.0 * f.rot_speed;
					f.active = true;
					f.energy += 1;
				} else {
					v.rot = 0.0;
				}

			}

		}

		e.set::<Vel>(v);
		e.set::<Flower>(f);

	}

}

fn debug(s: &mut World) {

	if !app::debug() {
		return;
	}

	for id in s.filter(comp_filter![Body]) {

		let e = s.get(id).unwrap();
		let b = e.get::<Body>();

		gfx::push();
		gfx::reset();
		gfx::line_width(1);
		gfx::color(color!(0, 1, 1, 1));
		gfx::poly(&b.d_verts);
		gfx::pop();

	}

}

fn petal_follow(s: &mut World) {

	for id in s.filter(comp_filter![Petal, Trans]) {

		let e = s.get(id).unwrap();
		let petal = e.get::<Petal>();
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

fn anim(s: &mut World) {

	for id in s.filter(comp_filter![Sprite, Trans]) {

		let e = s.get_mut(id).unwrap();
		let mut s = e.get::<Sprite>();

		if s.current_anim.is_some() {

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

fn render(s: &mut World) {

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
	let vel = Vel::default();
	let body = Body::new(&sprite.get_verts());
	let flower = Flower::new(p);

	sprite.color = flower.color;

	return entity![trans, sprite, vel, body, flower];

}

fn petal(flower: Id, index: u8) -> Entity {

	let trans = Trans::default();
	let mut sprite = Sprite::new("petal");
	let vel = Vel::default();
	let body = Body::new(&sprite.get_verts());
	let petal = Petal::new(flower, index);

	sprite.origin = vec2!(0.5, 1);

	return entity![trans, sprite, vel, body, petal];

}

fn powder(flower: Id, pos: Vec2, dir: f32) -> Entity {

	let sprite = Sprite::new("pixel");
	let trans = Trans::new(pos, 0.0, vec2!(1));
	let vel = Vel::new(vec2!(), 0.0, vec2!(1));
	let powder = Powder::new(flower, dir);

	return entity![sprite, trans, powder, vel];

}

fn rand_in_view(margin: u32) -> Vec2 {

	let (width, height) = window::size();

	return vec2!(rand!(margin, width - margin), rand!(margin, height - margin));

}

fn create_flower(s: &mut World, player: Player) {

	let f = s.add(flower(player, rand_in_view(24)));

	for i in 0..4 {
		s.add(petal(f, i));
	}

}

