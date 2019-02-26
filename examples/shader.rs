// wengwengweng

use dirty::*;

fn main() {

	// init modules
	app::init();
	audio::init();
	window::init("shader", 640, 480);
	res::init();

	let (w, h) = window::size();

	let canvas = gfx::Canvas::new(320, 320);
	let all = gfx::Canvas::new(w, h);
	let shader = g2d::Shader::from_code_frag(include_str!("assets/test.frag"));

	// main loop
	app::run(|| {

		g2d::color(color!(0, 1, 1, 1));
		g2d::rect(vec2!(w, h));

		g2d::set_shader(&shader);
		gfx::drawon(&all);
		g2d::translate(vec2!(160, 80));
		g2d::render(&canvas);
		g2d::render(&all);
		gfx::stop_drawon();


	});

}

