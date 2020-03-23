// wengwengweng

use dirty::*;
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

		ctx.draw_t(
			mat4!()
				.tz(-120.0)
				.s3(vec3!(64))
				.ry(ctx.time())
				.rz(ctx.time())
				,
			&shapes::cube()
		)?;

		ctx.draw(
			&shapes::text("yo")
				.size(16.0)
		)?;

		return Ok(());

	}

}

fn main() -> Result<()> {
	return run::<Game>();
}

