// wengwengweng

use dirty::*;
use window::Key;

struct Game;

impl app::State for Game {

	fn run(&mut self, ctx: &mut app::Ctx, dt: f32) {

		if window::key_pressed(ctx, Key::F) {
			window::toggle_fullscreen(ctx);
		}

		if window::key_pressed(ctx, Key::Escape) {
			app::quit(ctx);
		}

	}

}

fn main() {

	app::run_ex(Game, app::Conf {
		clear_color: color!(0, 0, 1, 1),
		..Default::default()
	}).expect("oh no");

}

