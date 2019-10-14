// wengwengweng

use dirty::*;
use dirty::audio::*;
use dirty::app::*;
use input::Key;

struct Game {
	track: Track,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		let sound = Sound::from_bytes(include_bytes!("res/yo.ogg")).unwrap();
		let track = Track::from(sound)?;

		track.play();

		return Ok(Self {
			track: track,
		});

	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				if k == Key::Esc {
					ctx.quit();
				}
				if k == Key::Space {
					if self.track.is_playing() {
						self.track.pause();
					} else {
						self.track.play();
					}
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, ctx: &mut app::Ctx) -> Result<()> {

		if self.track.is_playing() {
			ctx.draw(shapes::text("playing"))?;
		} else {
			ctx.draw(shapes::text("paused"))?;
		}

		return Ok(());

	}

}

fn main() {
	if let Err(err) = app::launcher()
		.run::<Game>() {
		println!("{}", err);
	}
}

