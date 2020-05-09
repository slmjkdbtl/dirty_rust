// wengwengweng

#![feature(box_syntax)]

use dirty::*;
use math::*;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn ui(&mut self, ctx: &mut Ctx, ui: &mut ui::UI) -> Result<()> {

		let top_left = ctx.coord(gfx::Origin::TopLeft);

		ui.window(ctx, "test", top_left + vec2!(64, -64), 240.0, 360.0, |ctx, p| {

			p.text(ctx, "yo")?;
			p.input(ctx, "name")?;
			p.slider(ctx, "age", 3.5, 1.0, 10.0)?;
			p.select(ctx, "gender", &["unknown", "male", "female"], 1)?;
			p.checkbox(ctx, "dead", false)?;
			p.sep(ctx)?;
			p.button(ctx, "explode")?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return run::<Game>();
}

