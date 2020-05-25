// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game {
	canvas1: gfx::Canvas,
	model: gfx::Model,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			canvas1: gfx::Canvas::new(d.gfx, 120, 160)?,
			model: gfx::Model::from_glb(d.gfx, include_bytes!("res/btfly.glb"))?,
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
// 			gfx.draw(&shapes::line(vec2!(-120), vec2!(120)))?;
			gfx.draw_t(mat4!().s3(vec3!(300)), &shapes::model(&self.model))?;
			return Ok(());
		})?;

		return Ok(());
	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {
// 		d.gfx.draw(&shapes::canvas(&self.canvas1))?;
		d.gfx.draw_t(mat4!().s3(vec3!(300)), &shapes::model(&self.model))?;
		return Ok(());
	}

}

fn main() {
	if let Err(e) = launcher()
		.resizable(true)
		.run::<Game>() {
		elog!("{}", e);
	}
}

