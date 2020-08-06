// wengwengweng

use std::time::Duration;

use dirty::*;
use audio::*;
use gfx::*;
use input::*;

struct Game {
	track: Track,
	sound: Sound,
}

impl State for Game {

	fn init(d: &mut Ctx) -> Result<Self> {

		let sound = Sound::from_bytes(d.audio, include_bytes!("res/shoot.ogg"))?;
		let track = Track::from_bytes(d.audio, include_bytes!("res/yo.ogg"))?;

		track.set_looping(true);

		return Ok(Self {
			sound,
			track,
		})

	}

	fn event(&mut self, d: &mut Ctx, e: &input::Event) -> Result<()> {

		match e {
			Event::KeyPress(k) => {
				match *k {
					Key::Esc => d.window.quit(),
					Key::Space => {
						if self.track.paused() {
							self.track.play();
						} else {
							self.track.pause();
						}
					},
					#[cfg(not(web))]
					_ => self.sound
						.builder()
						.chain(Delay::new(Duration::from_secs_f32(0.2), 3, 0.5))
						.pan(math::rand(0.0, 1.0), math::rand(0.0, 1.0))
						.volume(0.3)
						.play()?
						,
					#[cfg(web)]
					_ => self.sound.play(),
				}
			},
			_ => {},
		}

		return Ok(());

	}

	fn draw(&self, d: &mut Ctx) -> Result<()> {

		let time = d.app.time().as_secs_f32();
		let top_left = d.gfx.coord(gfx::Origin::TopLeft);

		#[cfg(not(web))]
		self.track.set_pan(math::wave(time, 0.0, 1.0), math::wave(time, 1.0, 0.0));

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

