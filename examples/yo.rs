// wengwengweng

use dirty::*;
use input::Key;

fn main() {

	// init modules
	app::init();
	audio::init();
	window::init("yo", 640, 480);
	res::init();

	let (width, height) = window::size();
	let mut index = 0;
	let margin = 16;
	let canvas = gfx::Canvas::new(width, height);
	let shader = g2d::Shader::from_code_frag(include_str!("assets/noise.frag"));

	// load resources
	res::load_textures_under("examples/assets/", &vec!["car"]);
	res::load_spritedata_under("examples/assets/", &vec!["car"]);
	res::load_sounds_under("examples/assets/", &vec!["pop", "yo"]);

	// play a music repeatedly
	let pop_sound = res::sound("pop");
	let music = audio::track(&res::sound("yo").repeat());

	let data = res::spritedata("car");
	let tex = res::texture("car");
	let frames = &data.frames;
	let anims = &data.anims;
	let mut hovering = false;

	let pts = vec![
		vec2!(0, 0) + vec2!(-margin, -margin),
		vec2!(tex.width() / 4, 0) + vec2!(margin, -margin),
		vec2!(tex.width() / 4, tex.height()) + vec2!(margin, margin),
		vec2!(0, tex.height()) + vec2!(-margin, margin),
	];

	// main loop
	app::run(|| {

		let time = app::time();
		let dt = app::dt();

		gfx::drawon(&canvas);
		gfx::clear();

		if hovering {
			g3d::rotate(vec3!(time));
			g3d::cube();
		}

		if index < anims["run"].to {
			index += 1;
		} else {
			index = anims["run"].from;
		}

		g2d::push();
		g2d::translate(vec2!(196, 164));
		g2d::scale(vec2!(2));
		g2d::translate(vec2!(64));
		g2d::rotate(((time * 2.0).sin() * 8.0).to_radians());
		g2d::translate(vec2!(-64));

		let pts = g2d::multi_warp(&pts);

		if hovering {
			g2d::color(color!(0, 1, 1, 1));
		} else {
			g2d::color(color!(1));
		}

		g2d::draw(&tex, frames[index]);
		g2d::pop();

		if col::point_poly(input::mouse_pos(), &pts) {
			hovering = true;
		} else {
			hovering = false;
		}

		if hovering {

			g2d::push();
			g2d::line_width(3);
			g2d::color(color!(1, 1, 0, 1));
			g2d::line(vec2!(rand!(width), rand!(height)), vec2!(rand!(width), rand!(height)));
			g2d::pop();

		}

		g2d::push();
		g2d::line_width(2);
		g2d::color(color!(1, 0, 1, 1));
		g2d::poly(&pts);
		g2d::pop();

		g2d::push();
		g2d::translate(vec2!(64, 64.0 + (time * 2.0).sin() * 4.0));
		g2d::scale(vec2!(3));
		g2d::text("yo♪");
		g2d::pop();

		g2d::push();
		g2d::translate(vec2!(16, 452));
		g2d::text("hover mouse over the car");
		g2d::pop();

		g2d::push();
		g2d::translate(vec2!(16));
		g2d::text(&format!("{}", app::fps()));
		g2d::pop();

		gfx::stop_drawon();
// 		g2d::set_shader(&shader);
		g2d::render(&canvas);
// 		g2d::set_shader_default();

		// inputs
		if input::key_pressed(Key::Space) {

			// play a sound with effect
			audio::play(&pop_sound.speed(rand!(2)));

		}

		if input::key_pressed(Key::Num1) {
			audio::pause(&music);
		}

		if input::key_pressed(Key::Num2) {
			audio::resume(&music);
		}

		if input::key_pressed(Key::F) {
			window::set_fullscreen(!window::get_fullscreen())
		}

		if input::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}

