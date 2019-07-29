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

		if ctx.key_pressed(Key::F) {
			ctx.toggle_fullscreen();
		}

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		ctx.scale(vec2!(12));
		ctx.draw(gfx::text("yo"))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::run::<Game>() {
		println!("{}", err);
	}

}

