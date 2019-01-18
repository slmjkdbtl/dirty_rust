// wengwengweng

use dirty::*;
use dirty::addons::res;
use dirty::addons::col;

fn main() {

	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

	let (width, height) = window::size();
	let mut index = 0;
	let margin = 16;
	let canvas = gfx::Canvas::new(width, height);

	res::load_sprites("examples/", &vec!["car"]);
	res::load_sounds("examples/", &vec!["pop", "yo"]);

	let music = audio::track(&res::sound("yo").repeat());

	let sprite = res::sprite("car");
	let tex = &sprite.tex;
	let frames = &sprite.frames;
	let anims = &sprite.anims;
	let mut tint = color!(1);

	let pts = vec![
		vec2!(0, 0) + vec2!(-margin, -margin),
		vec2!(tex.width() / 4, 0) + vec2!(margin, -margin),
		vec2!(tex.width() / 4, tex.height()) + vec2!(margin, margin),
		vec2!(0, tex.height()) + vec2!(-margin, margin),
	];

	app::run(&mut || {

		if index < anims["run"].to {
			index += 1;
		} else {
			index = anims["run"].from;
		}

		gfx::push();
		gfx::translate(vec2!(12));
		gfx::text(&format!("{}", app::fps()));
		gfx::pop();

		gfx::push();
		gfx::translate(vec2!(196, 164));
		gfx::scale(vec2!(2));
		gfx::translate(vec2!(64));
		gfx::rotate(((app::time() * 2.0).sin() * 8.0).to_radians());
		gfx::translate(vec2!(-64));

		let pts: Vec<Vec2> = pts.iter()
			.map(|&p| gfx::warp(p))
			.collect();

		gfx::color(tint);
		gfx::draw(&tex, frames[index]);
		gfx::pop();

		if col::point_poly(window::mouse_pos(), &pts) {

			tint = color!(0, 1, 1, 1);

			gfx::push();
			gfx::line_width(3);
			gfx::color(color!(1, 1, 0, 1));
			gfx::line(vec2!(rand!(width), rand!(height)), vec2!(rand!(width), rand!(height)));
			gfx::pop();

		} else {
			tint = color!(1);
		}

		gfx::push();
		gfx::line_width(1);
		gfx::color(color!(1, 0, 1, 1));
		gfx::poly(&pts);
		gfx::pop();

		gfx::push();
		gfx::translate(vec2!(64, 64.0 + (app::time() * 2.0).sin() * 4.0));
		gfx::scale(vec2!(3));
		gfx::text("yo♪");
		gfx::pop();

		if window::key_pressed(Key::Space) {
			audio::play(&res::sound("pop").speed(rand!(2)));
		}

		if window::key_pressed(Key::Num1) {
			audio::pause(&music);
		}

		if window::key_pressed(Key::Num2) {
			audio::resume(&music);
		}

		if window::key_pressed(Key::F) {
			window::set_fullscreen(!window::get_fullscreen())
		}

		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}

