// wengwengweng

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

		ctx.draw_masked(|ctx| {
			return ctx.draw(&shapes::circle(vec2!(0), 120.0));
		}, |ctx| {
			return ctx.draw(&shapes::gradient(
				vec2!(0, -120),
				vec2!(0, 120),
				&[
					(rgba!(0.4, 1, 1, 1), 0.0),
					(rgba!(1, 1, 0.6, 1), 0.5),
					(rgba!(1, 0.4, 0.8, 1), 1.0),
				],
			).width(240.0));
		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = launcher()
		.run::<Game>() {
		println!("{}", err);
	}
}

