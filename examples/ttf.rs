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

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::F => ctx.toggle_fullscreen(),
					Key::Esc => ctx.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		use shapes::*;

		ctx.push(
			mat4!()
				.s2(vec2!(12))
				,
			|ctx| {
				ctx.draw(
					&text("营养过剩")
						.font(&self.font)
						,
				)?;
				return Ok(());
			}
		)?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

