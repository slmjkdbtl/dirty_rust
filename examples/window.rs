// wengwengweng

use dirty::*;
use window::Key;

use dirty::app::Gfx;

const RATE: usize = 128;
const GATE: u16 = 54;

struct Game {

	tex: gfx::Texture,
	canvas: gfx::Canvas,
	count: usize,
	started: bool,
	done: bool,

}

impl app::State for Game {

	fn init(ctx: &mut window::Ctx) -> Result<Self> {

		return Ok(Self {
			tex: gfx::Texture::from_bytes(ctx, include_bytes!("res/icon.png"))?,
			canvas: gfx::Canvas::new(ctx, 640, 480)?,
			count: 0,
			done: false,
			started: false,
		});
	}

	fn run(&mut self, ctx: &mut window::Ctx) -> Result<()> {

		let w = 640;
		let h = 480;

		if ctx.key_pressed(Key::F) {
			ctx.toggle_fullscreen();
		}

		if ctx.key_pressed(Key::Escape) {
			ctx.quit();
		}

		if self.started {

			if !self.done {

				for _ in 0..self.count {

					ctx.push();
					ctx.translate(vec2!(rand!(0, w), rand!(0, h)));
					ctx.draw(gfx::sprite(&self.tex))?;
					ctx.pop()?;

				}

			} else {

				ctx.push();
				ctx.scale(vec2!(4));
				ctx.translate(vec2!(16));
				ctx.draw(gfx::text(&format!("{}", self.count)))?;
				ctx.pop()?;

			}

		} else {

			ctx.push();
			ctx.scale(vec2!(2));
			ctx.translate(vec2!(16));
			ctx.draw(gfx::text("waiting..."))?;
			ctx.pop()?;

		}

		ctx.set_title(&format!("FPS: {} DCS: {} OBJS: {}", ctx.fps(), ctx.draw_calls(), self.count));

		if !self.started {
			if ctx.fps() >= 60 {
				self.started = true;
			}
		} else {
			if !self.done {
				self.count += RATE;
				if ctx.fps() <= GATE {
					println!("{}", self.count);
					self.done = true;
				}
			}
		}

		return Ok(());

	}

}

fn main() {

	let result = app::App::new()
		.run::<Game>();

	if let Err(err) = result {
		println!("{}", err);
	}

}

