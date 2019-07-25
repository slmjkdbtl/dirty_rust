// wengwengweng

use dirty::*;
use window::Key;

struct Game {
}

impl Game {
	fn new(ctx: &mut app::Ctx) -> Result<Self> {
		return Ok(Self {
		});
	}
}

impl app::State for Game {

	fn run(&mut self, ctx: &mut app::Ctx, _dt: f32) -> Result<()> {

		if window::key_pressed(ctx, Key::F) {
			window::toggle_fullscreen(ctx);
		}

		if window::key_pressed(ctx, Key::Escape) {
			app::quit(ctx);
		}

		return Ok(());

	}

}

fn main() {
	app::run(Game::new, app::Conf::default()).expect("oh no");
}

