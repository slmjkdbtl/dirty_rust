// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game;

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		// TODO

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::run::<Game>() {
		println!("{}", err);
	}
}

