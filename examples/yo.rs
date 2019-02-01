// wengwengweng

use dirty::*;
use dirty::addons::res;
use dirty::addons::col;

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
	let shader = gfx::Shader::from_code_frag(include_str!("assets/noise.frag"));

	// load resources
	res::load_sprites("examples/assets/", &vec!["car"]);
	res::load_sounds("examples/assets/", &vec!["pop", "yo"]);

	// play a music repeatedly
	let music = audio::track(&res::sound("yo").repeat());

	let sprite = res::sprite("car");
	let tex = &sprite.tex;
	let frames = &sprite.frames;
	let anims = &sprite.anims;
	let mut tint = color!(1);

	let pts = vec![
		vec2!(0, 0) + vec2!(-margin, -margin),
		vec2!(tex.width() / 4, 0) + vec2!(margin, -margin),
		vec2!(tex.width() / 4, tex.height()) + vec2!(margin, margin),
		vec2!(0, tex.height()) + vec2!(-margin, margin),
	];

	gfx::drawon(&canvas);
	gfx::stop_drawon(&canvas);

	// main loop
	app::run(|| {

// 		gfx::render(&canvas);
// 		g3d::rotate(app::time(), app::time(), app::time());
// 		g3d::scale(vec3!(120));
// 		g3d::cube();

		if index < anims["run"].to {
			index += 1;
		} else {
			index = anims["run"].from;
		}

		gfx::push();
		gfx::translate(vec2!(196, 164));
		gfx::scale(vec2!(2));
		gfx::translate(vec2!(64));
		gfx::rotate(((app::time() * 2.0).sin() * 8.0).to_radians());
		gfx::translate(vec2!(-64));

		let pts = gfx::multi_warp(&pts);

		gfx::color(tint);
		gfx::draw(&tex, frames[index]);
		gfx::pop();

		if col::point_poly(window::mouse_pos(), &pts) {

			tint = color!(0, 1, 1, 1);

			gfx::push();
			gfx::line_width(3);
			gfx::color(color!(1, 1, 0, 1));
			gfx::line(vec2!(rand!(width), rand!(height)), vec2!(rand!(width), rand!(height)));
			gfx::pop();

		} else {
			tint = color!(1);
		}

		gfx::push();
		gfx::line_width(1);
		gfx::color(color!(1, 0, 1, 1));
		gfx::poly(&pts);
		gfx::pop();

		gfx::push();
		gfx::translate(vec2!(64, 64.0 + (app::time() * 2.0).sin() * 4.0));
		gfx::scale(vec2!(3));
		gfx::text("yo♪");
		gfx::pop();

		gfx::push();
		gfx::translate(vec2!(12));
		gfx::text(&format!("{}", app::fps()));
		gfx::pop();

		// inputs
		if window::key_pressed(Key::Space) {

			// play a sound with effect
			audio::play(&res::sound("pop").speed(rand!(2)));

		}

		if window::key_pressed(Key::Num1) {
			audio::pause(&music);
		}

		if window::key_pressed(Key::Num2) {
			audio::resume(&music);
		}

		if window::key_pressed(Key::F) {
			window::set_fullscreen(!window::get_fullscreen())
		}

		if window::key_pressed(Key::Escape) {
			app::quit();
		}

	});

}

