// wengwengweng

use dirty::*;
use input::Key;

struct Game {
	font: gfx::TruetypeFont,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let mut font = gfx::TruetypeFont::from_bytes(ctx, include_bytes!("res/Zpix.ttf"), 12)?;

		// TODO: temperarily have to cache manually
		font.cache_str("营养过剩")?;
		font.cache_str("1agyo+-好")?;

		return Ok(Self {
			font: font,
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

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

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		use shapes::*;

		ctx.push(
			mat4!()
				.s2(vec2!(12))
				,
			|ctx| {
				ctx.draw(
					&text("1agyo+-好")
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

	if let Err(err) = launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}

