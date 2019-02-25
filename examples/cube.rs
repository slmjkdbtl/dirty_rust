// wengwengweng

use dirty::*;

fn main() {

	// init modules
	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

	window::set_relative(true);

	let mut pos = vec3!(0, 0, 3);
	let mut rx: f32 = -88.0;
	let mut ry: f32 = 0.0;

	let move_speed = 4;
	let rot_speed = 0.2;

	// main loop
	app::run(|| {

		let time = app::time();
		let dt = app::dt();

		g3d::cam(pos);
		g3d::look(rx.to_radians(), ry.to_radians());
		g3d::rotate(vec3!(0, 2, 0));
// 		g3d::rotate(vec3!(time));
		g3d::cube();

		let md = window::mouse_delta();

		rx -= md.x * rot_speed;
		ry -= md.y * rot_speed;

		if ry > 24.0 {
			ry = 24.0;
		}

		if ry < -24.0 {
			ry = -24.0;
		}

		if window::key_down(Key::A) {
			pos = pos + g3d::get_front().cross(vec3!(0, 1, 0)).unit() * dt * move_speed;
		}

		if window::key_down(Key::D) {
			pos = pos - g3d::get_front().cross(vec3!(0, 1, 0)).unit() * dt * move_speed;
		}

		if window::key_down(Key::W) {
			pos = pos + g3d::get_front() * dt * move_speed;
		}

		if window::key_down(Key::S) {
			pos = pos - g3d::get_front() * dt * move_speed;
		}

		if window::key_pressed(Key::F) {
			window::set_fullscreen(!window::get_fullscreen())
		}

		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}


