// wengwengweng

use dirty::*;

fn main() {

	app::init();
	window::init("yo", 1280, 720);
	ui::init();

// 	window::set_fullscreen(true);

	let (width, height) = window::size();

	let test = ui::Window::new("window", vec2!(24, 24), 240, 320);

	ui::add(test);

	app::run(|| {

		gfx::color(color!(0.6, 0.78, 0.78, 1));
		gfx::rect(vec2!(width, height));
		ui::draw();

	});

}

