// wengwengweng

use std::sync::Arc;
use std::sync::Mutex;
use std::f32::consts::PI;

use dirty::*;
use app::*;
use input::Key;
use input::Mouse;

const NOTE_KEYS: [Key; 15] = [
	Key::A,
	Key::W,
	Key::S,
	Key::E,
	Key::D,
	Key::F,
	Key::T,
	Key::G,
	Key::Y,
	Key::H,
	Key::U,
	Key::J,
	Key::K,
	Key::O,
	Key::L,
];

const NOTE_OFFSET: i32 = -9;

struct Game {
	synth: Arc<Mutex<Synth>>,
	octave: i32,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		let synth = Arc::new(Mutex::new(Synth {
			wav: synth::Waveform::Triangle,
			volume: 0.1,
			notes: vec![],
			envelope: synth::Envelope {
				attack: 0.05,
				decay: 0.0,
				sustain: 1.0,
				release: 2.0,
			},
		}));

		synth::run(synth.clone())?;

		return Ok(Self {
			synth: synth,
			octave: 0,
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				if k == Key::Esc {
					ctx.quit();
				}

				if k == Key::Z {
					self.octave -= 1;
				}

				if k == Key::X {
					self.octave += 1;
				}

				let mut synth = self.synth.lock().unwrap();

				if let Some(index) = NOTE_KEYS.iter().position(|&x| x == k) {
					let e = synth.envelope;
					synth.notes.push(Note {
						freq: synth::get_note_freq(index as i32 + (NOTE_OFFSET + self.octave * 12)),
						life: synth::Note::new(e),
						key: k,
					});
				}

			},

			KeyRelease(k) => {

				let mut synth = self.synth.lock().unwrap();

				for n in &mut synth.notes {
					if n.key == k {
						n.release();
					}
				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let mut synth = self.synth.lock().unwrap();

		for n in &mut synth.notes {
			n.tick(ctx.dt());
		}

		synth.notes.retain(|n| !n.dead());

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		ctx.draw_t(&gfx::t()
			.t3(vec3!(0, 0, -6))
			.ry(ctx.time())
			.rz(ctx.time())
		, &shapes::cube())?;

		return Ok(());

	}

}

struct Note {
	life: synth::Note,
	freq: f32,
	key: Key,
}

impl Note {
	fn tick(&mut self, dt: f32) {
		self.life.update(dt);
	}
	fn sound(&self, wav: synth::Waveform, time: f32) -> f32 {
		return self.life.amp() * wav.osc(self.freq, time);
	}
	fn dead(&self) -> bool {
		return self.life.dead();
	}
	fn release(&mut self) {
		self.life.release();
	}
}

struct Synth {
	wav: synth::Waveform,
	volume: f32,
	envelope: synth::Envelope,
	notes: Vec<Note>,
}

impl synth::Stream for Synth {
	fn data(&mut self, time: f32) -> f32 {
		let mut sound = 0.0;
		for n in &mut self.notes {
			sound += n.sound(self.wav, time);
		}
		return sound * self.volume;
	}
}

fn main() -> Result<()> {
	return app::run::<Game>();
}

