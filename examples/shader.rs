// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game;

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
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

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		use gfx::Vertex;

		d.gfx.draw_with(&self.light_shader, &LightUniform {
			// ...
		}, |gfx| {

			gfx.draw_with(&self.m1_shader, &M1Uniform {
				// ...
			}, |gfx| {
				return Ok(());
			})?;

			gfx.draw_with(&self.m2_shader, &M2Uniform {
				// ...
			}, |gfx| {
				return Ok(());
			})?;

			return Ok(());

		})?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.title("raw")
		.run::<Game>() {
		elog!("{}", e);
	}
}

