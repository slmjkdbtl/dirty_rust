// wengwengweng

use sock::*;

fn main() {

	app::init();
	window::init("yo", 1280, 720);
	ui::init();

	let mut log = ui::Window::new("log", vec2!(48, 48), 240, 320);
	let mut game = ui::Window::new("game", vec2!(200, 160), 640, 480);
	let canvas = ui::Canvas::from_window(&game);
	let mut text_box = ui::TextBox::new();

	canvas.set(|| {

		g3d::rotate(app::time(), app::time(), app::time());
		g3d::scale(vec3!(120));
		g3d::cube();

	});

	text_box.write("yo");
	text_box.write("hello");

	log.add(text_box);
	game.add(canvas);

	ui::add(log);
	ui::add(game);

	app::run(|| {
		ui::draw();
	});

}

