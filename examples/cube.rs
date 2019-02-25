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
	let mut front = vec3!(0, 0, -1);
	let mut rx: f32 = -88.0;
	let mut ry: f32 = 0.0;

	// main loop
	app::run(|| {

		let time = app::time();
		let dt = app::dt();

		g3d::cam(pos);
		g3d::lookat(pos + front);
		g3d::rotate(vec3!(0, 2, 0));
// 		g3d::rotate(vec3!(time));
		g3d::cube();

		let md = window::mouse_delta();
		println!("{}", md);

		rx -= md.x * 0.5;
		ry -= md.y * 0.5;

		if ry > 89.0 {
			ry = 89.0;
		}

		if ry < -89.0 {
			ry = -89.0;
		}

		front.x = ry.to_radians().cos() * rx.to_radians().cos();
		front.y = ry.to_radians().sin();
		front.z = ry.to_radians().cos() * rx.to_radians().sin();
		front = front.unit();

		if window::key_down(Key::A) {
			pos = pos + front.cross(vec3!(0, 1, 0)).unit() * dt * 6;
		}

		if window::key_down(Key::D) {
			pos = pos - front.cross(vec3!(0, 1, 0)).unit() * dt * 6;
		}

		if window::key_down(Key::W) {
			pos = pos + front * dt * 6;
		}

		if window::key_down(Key::S) {
			pos = pos - front * dt * 6;
		}

		if window::key_pressed(Key::F) {
			window::set_fullscreen(!window::get_fullscreen())
		}

		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}


