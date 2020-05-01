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
			events: VecDeque::with_capacity(24),
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

		let top_left = ctx.coord(gfx::Origin::TopLeft);
		let mut y = 0.0;

		for (i, e) in self.events.iter().enumerate() {

			let size = i as f32 + 1.0;

			ctx.draw_t(
				mat4!()
					.t2(top_left + vec2!(0, y))
					,
				&shapes::text(&format!("{:?}", e))
					.align(gfx::Origin::TopLeft)
					.opacity(i as f32 / self.events.len() as f32)
					.size(size)
					,
			)?;

			y -= size;

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

