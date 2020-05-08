// wengwengweng

#![feature(box_syntax)]

use dirty::*;
use math::*;
use kit::ui::*;
use input::Key;

struct Game {
	ui: UI,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			ui: UI::new(),
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		self.ui.event(ctx, &e);

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

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let top_left = ctx.coord(gfx::Origin::TopLeft);

		self.ui.frame(|ui| {

			ui.panel(ctx, "test", top_left + vec2!(64, -64), 240.0, 320.0, |ctx, p| {

				p.text(ctx, "yo")?;
				p.input(ctx, "name")?;
				p.slider(ctx, "age", 3.5, 1.0, 10.0)?;

				return Ok(());

			})?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return run::<Game>();
}

