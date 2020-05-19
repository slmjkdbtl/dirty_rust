// wengwengweng

use dirty::*;
use audio::*;
use gfx::*;
use input::Key;

struct Game {
	track: Track,
	sound: Sound,
}

impl State for Game {

	fn init(ctx: &mut Ctx) -> Result<Self> {

		let audio = &ctx.audio;
		let sound = Sound::from_bytes(audio, include_bytes!("res/shoot.ogg"))?;
		let track = Track::from_bytes(audio, include_bytes!("res/yo.ogg"))?;

// 		track.play(audio);

		return Ok(Self {
			sound: sound,
			track: track,
		});

	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		let win = &mut ctx.window;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => win.quit(),
					Key::Space => {
						if self.track.is_playing() {
							self.track.pause();
						} else {
							self.track.play();
						}
					},
					_ => self.sound.play()?,
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let gfx = &mut ctx.gfx;

		if self.track.is_playing() {
			gfx.draw(&shapes::text("playing").size(16.0))?;
		} else {
			gfx.draw(&shapes::text("paused").size(16.0))?;
		}

		return Ok(());

	}

}

fn main() {
	if let Err(e) = launcher()
		.run::<Game>() {
		elog!("{}", e);
	}
}

