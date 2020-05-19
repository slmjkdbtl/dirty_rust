// wengwengweng

use dirty::*;
use gfx::shapes;
use input::Key;

struct Game {
	sound: audio::Sound,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {
		return Ok(Self {
			sound: audio::Sound::from_bytes(&ctx.audio, include_bytes!("res/yo.ogg"))?,
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		let win = &mut ctx.window;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => win.quit(),
					Key::L => self.sound.play(),
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		use gfx::Vertex;

		let gfx = &mut ctx.gfx;

		gfx.draw(&shapes::raw(&[
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
		.title("raw")
		.run::<Game>() {
		elog!("{}", e);
	}
}

