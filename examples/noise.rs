// wengwengweng

use dirty::*;
use math::*;
use gfx::shapes;
use input::Key;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, d: &mut Ctx) -> Result<()> {

		let mut x = 0.0;

		while x <= 640.0 {
// 			let nx1 = x.map(0.0, 640.0, 0.0, 640.0);
// 			let nx2 = (x + 1.0).map(0.0, 640.0, 0.0, 640.0);
			d.gfx.draw(
				&shapes::line(
					vec2!(x as i32 - 320, (noise(x) - 0.5) * 480.0),
					vec2!(x as i32 + 1 - 320, (noise(x + 1.0) - 0.5) * 480.0)
				)
			)?;
			x += 1.0;
		}

		return Ok(());

	}

}

fn main() {
	if let Err(e) = run::<Game>() {
		elog!("{}", e);
	}
}

