// wengwengweng

use dirty::*;
use gfx::*;
use input::*;

struct Game;

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(Self);
	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		match e {
			Event::KeyPress(k) => {
				match k {
					Key::Esc => d.window.quit(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, d: &mut Ctx) -> Result<()> {

		d.gfx.draw(&shapes::raw(&[
			Vertex {
				pos: vec3!(0, 72, 0),
				color: rgba!(1, 0, 0, 1),
				normal: vec3!(0, 0, 1),
				uv: vec2!(0),
			},
			Vertex {
				pos: vec3!(-96, -72, 0),
				color: rgba!(0, 1, 0, 1),
				normal: vec3!(0, 0, 1),
				uv: vec2!(0),
			},
			Vertex {
				pos: vec3!(96, -72, 0),
				color: rgba!(0, 0, 1, 1),
				normal: vec3!(0, 0, 1),
				uv: vec2!(0),
			},
		], &[0, 1, 2]))?;

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.run::<Game>() {
		elog!("{}", e);
	}
}

