// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	mask: gfx::Tex2D,
	tex: gfx::Tex2D,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			mask: gfx::Tex2D::from_bytes(ctx, include_bytes!("res/blob.png"))?,
			tex: gfx::Tex2D::from_bytes(ctx, include_bytes!("res/gradient.png"))?,
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

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		use gfx::Transform::*;

		ctx.push(&[

			Scale(vec2!(2))

		], |ctx| {

			ctx.draw_masked(|ctx| {
				return ctx.draw(shapes::sprite(&self.mask));
			}, |ctx| {
				return ctx.push(&[
					Translate(vec2!(0, (ctx.time() * 6.0).sin() * 24.0))
				], |ctx| {
					return ctx.draw(shapes::sprite(&self.tex));
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


