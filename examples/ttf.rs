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

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		self.font.draw(ctx, "营养过剩")?;

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.origin(gfx::Origin::TopLeft)
		.texture_origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}

}

