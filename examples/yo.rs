// wengwengweng

#![windows_subsystem = "windows"]

use dirty::*;
use dirty::math::*;

fn main() {

	app::init("yo", 640, 480);

	let tex = gfx::make_tex(&include_bytes!("./car.png")[..]);
	let mut index = 0;

	res::load_sprites(".", vec!["car"]);

	app::run(&mut || {

		if index < 3 {
			index += 1;
		} else {
			index = 0;
		}

		gfx::clear();

		gfx::translate(vec2!(100));
		gfx::scale(vec2!(2));
		gfx::rotate(16.0f32.to_radians());
		gfx::draw(&tex, rect!((index as f32) * 0.25, 0, 0.25, 1));
		gfx::text("yo");
		gfx::color(color!(1, 1, 0, 1));
		gfx::line(vec2!(0), vec2!(640, 480));

		if app::key_pressed(Key::F) {
			app::set_fullscreen(!app::get_fullscreen())
		}

		if app::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}

