// wengwengweng

use dirty::*;
use dirty::app::*;
use input::Key;

const RATE: usize = 128;
const GATE: u16 = 54;

struct Game {

	tex: gfx::Texture,
	count: usize,
	started: bool,
	done: bool,

}

impl app::State for Game {

	fn init(ctx: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			tex: gfx::Texture::from_bytes(ctx, include_bytes!("res/icon.png"))?,
			count: 0,
			done: false,
			started: false,
		});
	}

	fn run(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let w = ctx.width() as i32;
		let h = ctx.height() as i32;

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
					ctx.translate(vec2!(rand!(-w / 2, w / 2), rand!(-h / 2, h / 2)));
					ctx.draw(shapes::sprite(&self.tex))?;
					ctx.pop()?;

				}

			} else {

				ctx.push();
				ctx.scale(vec2!(6));
				ctx.draw(shapes::text(&format!("{}", self.count)))?;
				ctx.pop()?;

			}

		} else {

			ctx.push();
			ctx.scale(vec2!(2));
			ctx.draw(shapes::text("waiting..."))?;
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

	if let Err(err) = app::run::<Game>() {
		println!("{}", err);
	}

}

