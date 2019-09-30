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
			font: gfx::TrueTypeFont::new(ctx, include_bytes!("res/Zpix.ttf"), 64.0)?,
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw(&shapes::text("营养过剩").font(gfx::Font::TrueType(&mut self.font)))?;

		ctx.push(&gfx::t()
			.translate(vec2!(120))
		, |ctx| {
			return ctx.draw(&shapes::text("你是谁").font(gfx::Font::TrueType(&mut self.font)));
		});

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

