// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw_masked(|gfx| {
			return gfx.draw(&shapes::circle(vec2!(0), 120.0));
		}, |gfx| {
			return gfx.draw(&shapes::gradient(
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
	if let Err(e) = launcher()
		.run::<Game>() {
		log!("{}", e);
	}
}

