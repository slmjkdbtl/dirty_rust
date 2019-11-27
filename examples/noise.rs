// wengwengweng

use dirty::*;
use app::*;
use input::Key;

use math::NoiseFn;
use math::Seedable;

struct Game {
	perlin: math::Perlin,
	seed: u32,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			perlin: math::Perlin::new(),
			seed: 0,
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => ctx.quit(),
					Key::Space => {
						self.seed += 1;
						self.perlin.set_seed(self.seed);
					},
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let gw = ctx.gwidth() as i32;
		let mut last_pt = None;

		for x in -gw..gw as i32 {

			let y = self.perlin.get([x as f64 / 100.0 + ctx.time() as f64; 2]) * 120.0;
			let pt = vec2!(x, y);

			if let Some(last_pt) = last_pt {
				ctx.draw(&shapes::line(last_pt, pt))?;
			}

			last_pt = Some(pt);

		}

		return Ok(());

	}

}

fn main() -> Result<()> {
	return launcher()
		.scale(2.0)
		.run::<Game>();
}

