// wengwengweng

use std::collections::VecDeque;

use dirty::*;
use input::Key;

struct Game {
	events: VecDeque<input::Event>,
}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			events: VecDeque::with_capacity(12),
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		self.events.push_back(e.clone());

		if self.events.len() >= self.events.capacity() {
			self.events.pop_front();
		}

		match *e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		for (i, e) in self.events.iter().enumerate() {
			ctx.push(mat4!()
				.t2(ctx.coord(gfx::Origin::TopLeft) + vec2!(0, (i as i32) * -24))
				.s2(vec2!(i) / 6.0)
			, |ctx| {
				ctx.draw(
					&shapes::text(&format!("{:?}", e))
						.align(gfx::Origin::TopLeft)
						.opacity(i as f32 / self.events.len() as f32)
				)?;
				return Ok(());
			})?;
		}

		return Ok(());

	}

}

fn main() {
	if let Err(err) = launcher()
		.run::<Game>() {
		println!("{}", err);
	}
}

