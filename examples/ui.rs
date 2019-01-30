// wengwengweng

use dirty::*;
use dirty::addons::res;
use dirty::addons::col;

fn main() {

	app::init();
	window::init("yo", 1280, 720);
	res::init();
	ui::init();

	res::load_sprites("examples/assets/", &vec!["car"]);
	res::load_sounds("examples/assets/", &vec!["pop", "yo"]);

	let (width, height) = window::size();
	let mut index = 0;
	let margin = 16;

	let mut log = ui::Window::new("log", vec2!(48, 48), 240, 320);
	let mut game = ui::Window::new("game", vec2!(200, 160), 640, 480);
	let canvas = ui::Canvas::from_window(&game);
	let mut text_box = ui::TextBox::new();

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

	canvas.set(|| {

		if index < anims["run"].to {
			index += 1;
		} else {
			index = anims["run"].from;
		}

		gfx::push();
		gfx::translate(vec2!(12));
		gfx::text(&format!("{}", app::fps()));
		gfx::pop();

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

