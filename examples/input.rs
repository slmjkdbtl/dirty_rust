// wengwengweng

use std::collections::VecDeque;

use dirty::*;
use dirty::app::*;
use input::Key;

struct Game {
	events: VecDeque<input::Event>,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {
		return Ok(Self {
			events: VecDeque::with_capacity(12),
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		self.events.push_back(e.clone());

		if self.events.len() >= self.events.capacity() {
			self.events.pop_front();
		}

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

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		for (i, e) in self.events.iter().enumerate() {
			ctx.push(&gfx::t()
				.scale(vec2!(i) / 3.0)
				.translate(vec2!(0, i * 6))
			, |ctx| {
				ctx.draw(
					shapes::text(&format!("{:?}", e))
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
	if let Err(err) = app::launcher()
		.origin(gfx::Origin::TopLeft)
		.run::<Game>() {
		println!("{}", err);
	}
}

