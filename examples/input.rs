// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game;

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
			},
			GamepadPress(id, k) => {
				dbg!(id, k);
			},
			_ => {},
		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw(&shapes::text("yo"))?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::run::<Game>() {
		println!("{}", err);
	}
}

