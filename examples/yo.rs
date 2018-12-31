// wengwengweng

#![windows_subsystem = "windows"]

use dirty::*;
use dirty::math::*;

fn main() {

	app::init("yo", 640, 480);

	let tex = gfx::Texture::from_bytes(&include_bytes!("./car.png")[..]);
	let canvas = gfx::Canvas::new();
	let mut index = 0;
	let margin = 16;

	let pts = vec![
		vec2!(0, 0) + vec2!(-margin, -margin),
		vec2!(tex.width / 4, 0) + vec2!(margin, -margin),
		vec2!(tex.width / 4, tex.height) + vec2!(margin, margin),
		vec2!(0, tex.height) + vec2!(-margin, margin),
	];

	res::load_sprites(".", vec!["car"]);

	app::run(&mut || {

		let (width, height) = app::size();

		if index < 3 {
			index += 1;
		} else {
			index = 0;
		}

		gfx::clear();

// 		gfx::draw_on(&canvas);
		gfx::push();
		gfx::translate(vec2!(196, 164));
		gfx::scale(vec2!(2));
		gfx::translate(vec2!(64));
		gfx::rotate(((app::time() * 0.2).sin() * 8.0).to_radians());
		gfx::translate(vec2!(-64));

		let pts: Vec<Vec2> = pts.iter()
			.map(|&p| gfx::warp(p))
			.collect();

		gfx::draw(&tex, rect!((index as f32) * 0.25, 0, 0.25, 1));
		gfx::pop();

		gfx::line_width(3);
		gfx::color(color!(1, 1, 0, 1));
		gfx::line(rand_vec2() * vec2!(width, height), rand_vec2() * vec2!(width, height));

		gfx::line_width(1);
		gfx::color(color!(1, 0, 1, 1));
		gfx::poly(pts);

		gfx::color(color!(1));
		gfx::push();
		gfx::translate(vec2!(64, 64.0 + (app::time() * 0.2).sin() * 4.0));
		gfx::scale(vec2!(3));
		gfx::text("yo♪");
		gfx::pop();
// 		gfx::stop_draw_on(&canvas);

// 		gfx::render(&canvas);

		if app::key_pressed(Key::F) {
			app::set_fullscreen(!app::get_fullscreen())
		}

		if app::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}

