// wengwengweng

#![windows_subsystem = "windows"]
#![allow(unused_parens)]

use dirty::*;
use dirty::math::*;
use dirty_addons::*;

fn main() {

	app::init("yo", 640, 480);

	let tex = gfx::Texture::from_bytes(&include_bytes!("car.png")[..]);
	let mut index = 0;

	app::run(&mut || {

		if (index < 3) {
			index += 1;
		} else {
			index = 0;
		}

		gfx::clear();
		gfx::draw(&tex, vec2(240.0, 240.0), 0.0, vec2(2.0, 2.0), vec4((index as f32) * 0.25, 0.0, 0.25, 1.0), vec4(1.0, 1.0, 1.0, 1.0));

	});

}

