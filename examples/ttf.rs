// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	font: gfx::TruetypeFont,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut font = gfx::TruetypeFont::from_bytes(ctx, include_bytes!("res/Zpix.ttf"), 12)?;

		// TODO: temperarily have to cache manually
		font.cache("营养过剩")?;

		return Ok(Self {
			font: font,
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		use shapes::*;

		ctx.push(&gfx::t()
			.scale(vec2!(12))
		, |ctx| {
			ctx.draw(
				&text("营养过剩")
					.font(&self.font)
			)?;
			return Ok(());
		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}

}

