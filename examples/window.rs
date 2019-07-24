// wengwengweng

use dirty::*;
use dirty::window::Key;

struct Game;

impl app::State for Game {

	fn run(&mut self, ctx: &mut app::Ctx) {

		if window::key_pressed(ctx, Key::F) {
			window::toggle_fullscreen(ctx);
		}

		if window::key_pressed(ctx, Key::Escape) {
			app::quit(ctx);
		}

	}

}

fn main() {
	app::run_with_conf(Game, app::Conf {
		clear_color: color!(0, 0, 1, 1),
		..Default::default()
	}).expect("oh no");
}

