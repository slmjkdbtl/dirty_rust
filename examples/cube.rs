// wengwengweng

use dirty::*;
use dirty::math::*;
use window::Key;

fn main() {

	// init modules
	app::init();
	window::init(window::Conf::basic("cube", 640, 480));
	window::set_relative(true);

	let mut pos = vec3!(9, 9, 9);
	let mut rx: f32 = -135.0;
	let mut ry: f32 = -35.0;

	let move_speed = 6.0;
	let rot_speed = 0.2;

	let size = 3;

	// main loop
	app::run(|| {

		let time = app::time();
		let dt = app::dt();

		g3d::cam(pos);
		g3d::look(rx.to_radians(), ry.to_radians());

		for x in -size..size {
			for y in -size..size {
				for z in -size..size {

					g3d::push();
					g3d::translate(vec3!(x, y, z));
					g3d::rotate(vec3!(app::time()));
					g3d::cube();
					g3d::pop();

				}
			}
		}

		g2d::push();
		g2d::translate(vec2!(16));
		g2d::text(&format!("{}", app::fps()));
		g2d::pop();

		if let Some(scroll) = window::scroll_delta() {
			// ...
		}

		if window::is_relative() {

			if let Some(delta) = window::mouse_delta() {

				let md: Vec2 = delta.into();

				rx -= md.x * rot_speed;
				ry -= md.y * rot_speed;

				if ry > 48.0 {
					ry = 48.0;
				}

				if ry < -48.0 {
					ry = -48.0;
				}

			}

		}

		if window::key_down(Key::A) {
			pos = pos + g3d::front().cross(vec3!(0, 1, 0)).unit() * dt * move_speed;
		}

		if window::key_down(Key::D) {
			pos = pos - g3d::front().cross(vec3!(0, 1, 0)).unit() * dt * move_speed;
		}

		if window::key_down(Key::W) {
			pos = pos + g3d::front() * dt * move_speed;
		}

		if window::key_down(Key::S) {
			pos = pos - g3d::front() * dt * move_speed;
		}

		if window::key_pressed(Key::F) {
			window::set_fullscreen(!window::is_fullscreen())
		}

		if window::key_pressed(Key::Escape) {
			window::set_relative(!window::is_relative());
		}

	});

}

