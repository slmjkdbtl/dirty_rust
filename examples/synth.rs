// wengwengweng

use std::sync::Arc;
use std::sync::Mutex;
use std::f32::consts::PI;

use dirty::*;
use app::*;
use input::Key;
use input::Mouse;

const NOTE_KEYS: [Key; 13] = [
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
];

const NOTE_OFFSET: i32 = -9;

struct Game {
	synth: Arc<Mutex<Synth>>,
	note: Option<synth::Note>,
	envelope: synth::Envelope,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		let synth = Arc::new(Mutex::new(Synth {
			freq: 0.0,
			waveform: synth::Waveform::Saw,
			volume: 0.1,
		}));

		synth::run(synth.clone())?;

		return Ok(Self {
			synth: synth,
			note: None,
			envelope: synth::Envelope {
				attack: 0.0,
				decay: 1.0,
				sustain: 0.0,
				release: 0.0,
			},
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				if k == Key::Esc {
					ctx.quit();
				}

				let mut synth = self.synth.lock().unwrap();

				if let Some(index) = NOTE_KEYS.iter().position(|&x| x == k) {
					synth.freq = synth::get_note_freq(index as i32 + NOTE_OFFSET);
					self.note = Some(synth::Note::new(self.envelope));
				}

			},

			KeyRelease(k) => {
				if let Some(note) = &mut self.note {
					note.release();
				}
			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		let mut synth = self.synth.lock().unwrap();

		if let Some(note) = &mut self.note {
			note.update(ctx.dt());
			synth.volume = note.amp();
			if note.dead() {
				self.note = None;
			}
		}

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

struct Synth {
	freq: f32,
	waveform: synth::Waveform,
	volume: f32,
}

impl synth::Stream for Synth {
	fn data(&self, dt: f32) -> f32 {
		return synth::osc(self.waveform, self.freq, dt) * self.volume;
	}
}

fn main() -> Result<()> {
	return app::run::<Game>();
}

