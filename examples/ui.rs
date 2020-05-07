// wengwengweng

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

			ui.panel(ctx, "yo", top_left + vec2!(64, -64), 240.0, 160.0, |ctx, p| {

				p.text(ctx, "text1")?;
				p.text(ctx, "text2")?;
				p.input(ctx, "name")?;

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

