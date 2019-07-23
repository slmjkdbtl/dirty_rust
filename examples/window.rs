// wengwengweng

use dirty::*;
use dirty::window::Key;

struct Game;

impl app::State for Game {

	fn init(&mut self, ctx: &mut app::Ctx) {
		gfx::clear_color(ctx, color!(0, 0, 1, 1));
	}

	fn run(&mut self, ctx: &mut app::Ctx) {
		if window::key_pressed(ctx, Key::Escape) {
			app::quit(ctx);
		}
	}

}

fn main() {
	app::run(Game);
}

