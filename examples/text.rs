// wengwengweng

use dirty::*;
use app::*;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let size = 14.0;
		let t = (ctx.time() * 2.0) as i32;

		ctx.draw(
			&shapes::Rect::from_size(64.0, 64.0)
				.no_fill()
				.stroke(rgba!(1))
		)?;

		let s = format!("{}", t).repeat(12);

		let text = shapes::text(&s)
			.wrap(64.0, true)
			.size(size)
// 			.align(gfx::Origin::TopLeft)
			.render(ctx)
			;

		let cpos = text.cursor_pos(ctx, t);

		ctx.draw(
			&text
		)?;

		if let Some(cpos) = cpos {
			ctx.draw(
				&shapes::line(cpos, cpos + vec2!(0, size))
			)?;
		}

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Game>();
}

