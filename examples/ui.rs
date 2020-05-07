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

		self.ui.frame(ctx, |ui| {

			ui.panel("yo", top_left + vec2!(24, -24), 240.0, 160.0, |p| {

				p.text("yo");
				p.text("sup");

			});

		});

		return Ok(());

	}

}

fn main() -> Result<()> {
	return run::<Game>();
}

