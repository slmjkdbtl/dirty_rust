// wengwengweng

use dirty::*;

fn main() {

	// init modules
	app::init();
	window::init("yo", 640, 480);

	// main loop
	app::run(|| {

		g3d::rotate(app::time(), app::time(), app::time());
		g3d::scale(vec3!(120));
		g3d::cube();

	});

}

