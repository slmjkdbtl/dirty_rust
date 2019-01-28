// wengwengweng

use dirty::*;

fn main() {

	app::init();
	window::init("yo", 1280, 720);
	ui::init();

// 	window::set_fullscreen(true);

	let (width, height) = window::size();

	let mut log = ui::Window::new("log", vec2!(48, 48), 240, 320);
	let mut game = ui::Window::new("game", vec2!(200, 160), 640, 480);
	let mut text_box = ui::TextBox::new();
	let canvas = ui::Canvas::new(&game);

	text_box.write("yo");
	text_box.write("hello");

	log.add(text_box);
	game.add(canvas);

	ui::add(log);
	ui::add(game);

	app::run(|| {

		gfx::color(color!(0.6, 0.78, 0.78, 1));
		gfx::rect(vec2!(width, height));
		ui::draw();

	});

}

