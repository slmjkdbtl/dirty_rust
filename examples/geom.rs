// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game;

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
				if *k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		use shapes::*;

		ctx.draw(gradient(
			ctx.coord(gfx::Origin::Top),
			ctx.coord(gfx::Origin::Bottom),
			&[
				(color!(0.4, 1, 1, 1), 0.0),
				(color!(1, 1, 0.6, 1), 0.5),
				(color!(1, 0.4, 0.8, 1), 1.0),
			],
		).width(640.0))?;

		ctx.draw(circle(vec2!(0), 120.0).color(color!(1, 0, 1, 1)))?;
		ctx.draw(rect(vec2!(-64, -48), vec2!(64, 48)).color(color!(0, 1, 1, 1)))?;
		ctx.draw(rect(vec2!(-48, -32), vec2!(48, 32)).color(color!(1, 1, 0, 1)))?;
		ctx.draw(text("geoms").color(color!(0, 0, 1, 1)))?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::run::<Game>() {
		println!("{}", err);
	}
}

