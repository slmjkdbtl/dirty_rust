// wengwengweng

use dirty::*;
use audio::*;
use gfx::*;
use input::Key;

struct Game {
	track: Track,
// 	sound: Sound,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

// 		let sound = Sound::from_bytes(d.audio, include_bytes!("res/shoot.ogg"))?;
		let track = Track::from_bytes(d.audio, include_bytes!("res/yo.ogg"))?;

		return Ok(Self {
// 			sound: sound,
			track: track,
		});

	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {
			KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::Space => {
						if self.track.paused() {
							self.track.play();
						} else {
							self.track.pause();
						}
					},
// 					_ => self.sound.play()?,
					_ => {},
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&mut self, d: &mut Ctx) -> Result<()> {

		let top_left = d.gfx.coord(gfx::Origin::TopLeft);

		let lines = [
			"space:   play / pause",
			"any key: play one shot sounds",
		];

		for (i, l) in lines.iter().enumerate() {
			d.gfx.draw_t(
				mat4!()
					.t2(top_left + vec2!(24, -24.0 - i as f32 * 24.0))
					,
				&shapes::text(l)
					.align(gfx::Origin::TopLeft)
					.size(12.0)
					,
			)?;
		}

		if self.track.paused() {
			d.gfx.draw(&shapes::text("paused").size(16.0))?;
		} else {
			d.gfx.draw(&shapes::text("playing").size(16.0))?;
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

