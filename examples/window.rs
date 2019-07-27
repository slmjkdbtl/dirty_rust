// wengwengweng

use dirty::*;
use window::Key;

const RATE: usize = 96;
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

	fn run(&mut self, ctx: &mut app::Ctx, dt: f32) -> Result<()> {

		let w = 640;
		let h = 480;

		if window::key_pressed(ctx, Key::F) {
			window::toggle_fullscreen(ctx);
		}

		if window::key_pressed(ctx, Key::Escape) {
			app::quit(ctx);
		}

		if !self.done {
			for _ in 0..self.count {
				gfx::draw(ctx, &self.tex, vec2!(rand!(-w / 2, w / 2), rand!(-h / 2, h)), 0.0, vec2!(1), rect!(0, 0, 1, 1), color!())?;
			}
		}

		window::set_title(ctx, &format!("FPS: {} DCS: {} OBJS: {}", app::fps(ctx), gfx::draw_calls(ctx), self.count));

		if !self.started {
			if app::fps(ctx) >= 60 {
				self.started = true;
			}
		} else {
			if !self.done {
				self.count += RATE;
				if app::fps(ctx) <= GATE {
					println!("{}", self.count);
					self.done = true;
				}
			}
		}

		return Ok(());

	}

}

fn main() {

	let result = app::run::<Game>(app::Conf::default());

	if let Err(err) = result {
		println!("{}", err);
	}

}

