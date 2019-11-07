rgba!gwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	mask: gfx::Texture,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			mask: gfx::Texture::from_bytes(ctx, include_bytes!("res/blob.png"))?,
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

		ctx.push(&gfx::t()
			.scale(vec2!(2))
		, |ctx| {

			ctx.draw_masked(|ctx| {
				return ctx.draw(&shapes::sprite(&self.mask));
			}, |ctx| {
				return ctx.push(&gfx::t()
					.translate(vec2!(0, (ctx.time() * 6.0).sin() * 24.0))
				, |ctx| {
					return ctx.draw(&shapes::gradient(
						vec2!(0, -80),
						vec2!(0, 80),
						&[
							(rgba!(0.4, 1, 1, 1), 0.0),
							(rgba!(1, 1, 0.6, 1), 0.5),
							(rgba!(1, 0.4, 0.8, 1), 1.0),
						],
					).width(160.0));
				});
			})?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {

	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}

}


