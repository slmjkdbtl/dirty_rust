// wengwengweng

use dirty::*;
use dirty::*;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match *e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		ctx.draw(&shapes::text("yo"))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = launcher()
		.size(640, 480)
		.title("")
		.hidpi(true)
		.resizable(false)
		.fullscreen(false)
		.vsync(true)
		.cursor_hidden(false)
		.cursor_locked(false)
		.transparent(false)
		.always_on_top(false)
		.fps_cap(Some(60))
		.run::<Game>() {
		println!("{}", err);
	}
}

