// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game {
	canvas1: gfx::Canvas,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			canvas1: gfx::Canvas::new(d.gfx, 100, 100)?,
		});
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::Key1 => self.canvas1.capture()?.save("1.png")?,
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn update(&mut self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw_on(&self.canvas1, |gfx| {
			gfx.draw(&shapes::rect(vec2!(0), vec2!(50, -50)))?;
			return Ok(());
		})?;

		return Ok(());
	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {
		d.gfx.draw(&shapes::canvas(&self.canvas1))?;
		return Ok(());
	}

}

fn main() {
	if let Err(e) = run::<Game>() {
		elog!("{}", e);
	}
}

