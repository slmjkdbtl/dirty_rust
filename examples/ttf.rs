// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	font: gfx::TruetypeFont,
	tex: gfx::Texture,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let mut font = gfx::TruetypeFont::from_bytes(ctx, include_bytes!("res/Zpix.ttf"), 120.0)?;
		let tex = font.get_char_tex(ctx, '我')?;

		font.prepare("123123");

		return Ok(Self {
			font: font,
			tex: tex,
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

		ctx.draw(shapes::sprite(&self.tex))?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
// 		.origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}

}

