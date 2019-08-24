// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	font: gfx::TrueTypeFont,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {
		return Ok(Self {
			font: gfx::TrueTypeFont::new(ctx, include_bytes!("res/Zpix.ttf"), 130.0)?,
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Escape {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.font.draw(ctx, "营养过剩")?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.origin(gfx::Origin::TopLeft)
		.quad_origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}

}

