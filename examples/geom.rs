// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	pixel_effect: gfx::Shader,
	canvas: gfx::Canvas,
}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		let pixel_effect = gfx::Shader::effect(ctx, include_str!("res/pix.frag"))?;

		pixel_effect.send("size", 6.0);
		pixel_effect.send("dimension", vec2!(ctx.width(), ctx.height()));

		return Ok(Self {
			pixel_effect: pixel_effect,
			canvas: gfx::Canvas::new(ctx, ctx.width(), ctx.height())?,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if *k == Key::Esc {
					ctx.quit();
				}
				if *k == Key::F {
					ctx.toggle_fullscreen();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_on(&self.canvas, |ctx| {

			ctx.draw(shapes::gradient(
				ctx.coord(gfx::Origin::Top),
				ctx.coord(gfx::Origin::Bottom),
				&[
					(color!(0, 1, 1, 1), 0.0),
					(color!(1, 0, 1, 1), 0.5),
					(color!(0, 0, 1, 1), 0.75),
					(color!(1, 1, 1, 1), 1.0),
				],
			).width(640.0))?;

			ctx.draw(
				shapes::polygon(&[
					vec2!(0, 0),
					vec2!(120, 96),
					vec2!(64, 160),
				])
					.stroke(12.0)
					.line_join(gfx::LineJoin::Round)
			)?;

			return Ok(());

		})?;

// 		ctx.draw_with(&self.pixel_effect, |ctx| {
			return ctx.draw(shapes::canvas(&self.canvas));
// 		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::run::<Game>() {
		println!("{}", err);
	}
}

