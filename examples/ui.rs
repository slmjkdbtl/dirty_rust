// wengwengweng

use dirty::*;

fn main() {

	app::init();
	window::init("yo", 640, 480);

	let (width, height) = window::size();

	app::run(|| {

		gfx::color(color!(0.6, 0.78, 0.78, 1));
		gfx::rect(vec2!(width, height));
		gfx::translate(vec2!(24));
		ui::window("window", 240, 320);

	});

}

