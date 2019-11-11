// wengwengweng

#![feature(clamp)]

use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;

use dirty::*;
use app::*;
use input::Key;

const NOTE_KEYS: [Key; 17] = [
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
	Key::P,
	Key::Semicolon,
];

const NOTE_OFFSET: i32 = -9;

struct Game {
	synth: Arc<Mutex<Synth>>,
	octave: i32,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		let mut synth = Synth::new();

		synth.set_volume(0.2);
		synth.set_waveform(synth::Waveform::Saw);

		let synth = Arc::new(Mutex::new(synth));

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

				if k == Key::Right {
					self.octave += 1;
				}

				if k == Key::Left {
					self.octave -= 1;
				}

				if let Ok(mut synth) = self.synth.lock() {

					let vol = synth.volume();

					if k == Key::Up {
						synth.set_volume(vol + 0.1);
					}

					if k == Key::Down {
						synth.set_volume(vol - 0.1);
					}

					if k == Key::Key1 {
						synth.set_waveform(synth::Waveform::Sine);
					}

					if k == Key::Key2 {
						synth.set_waveform(synth::Waveform::Triangle);
					}

					if k == Key::Key3 {
						synth.set_waveform(synth::Waveform::Square);
					}

					if k == Key::Key4 {
						synth.set_waveform(synth::Waveform::Saw);
					}

					if k == Key::Key5 {
						synth.set_waveform(synth::Waveform::Noise);
					}

					if let Some(index) = NOTE_KEYS.iter().position(|&x| x == k) {

						let e = synth.envelope;

						synth.notes.push(Note {
							freq: synth::get_note_freq(index as i32 + (NOTE_OFFSET + self.octave * 12)),
							life: synth::Note::new(e),
							key: k,
						});

					}

				}

			},

			KeyRelease(k) => {

				if let Ok(mut synth) = self.synth.lock() {
					for n in &mut synth.notes {
						if n.key == k {
							n.release();
						}
					}
				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn update(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if let Ok(mut synth) = self.synth.lock() {

			for n in &mut synth.notes {
				n.tick(ctx.dt());
			}

			synth.notes.retain(|n| !n.dead());

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if let Ok(synth) = self.synth.lock() {

			let mut last = None;
			let height = 240.0;
			let len = synth.buf.len() as f32;
			let dis = ctx.gwidth() as f32 / len;

			for (i, buf) in synth.buf.iter().enumerate() {

				if let Some(last) = last {

					let r = i as f32 / len;
					let ay = last * height;
					let by = buf * height;
					let ax = -len / 2.0 * dis + (i - 1) as f32 * dis;
					let bx = -len / 2.0 * dis + i as f32 * dis;

					ctx.draw(&shapes::line(vec2!(ax, ay), vec2!(bx, by)))?;

				}

				last = Some(buf);

			}

		}

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
	waveform: synth::Waveform,
	volume: f32,
	envelope: synth::Envelope,
	notes: Vec<Note>,
	buf: VecDeque<f32>,
}

impl Synth {

	fn new() -> Self {
		return Synth {
			waveform: synth::Waveform::Sine,
			volume: 1.0,
			notes: vec![],
			buf: VecDeque::with_capacity(100),
			envelope: synth::Envelope {
				attack: 0.05,
				decay: 0.0,
				sustain: 1.0,
				release: 2.0,
			},
		};
	}

	fn volume(&self) -> f32 {
		return self.volume;
	}

	fn set_volume(&mut self, v: f32) {
		self.volume = v.clamp(0.0, 1.0);
	}

	fn set_envelope(&mut self, e: synth::Envelope) {
		self.envelope = e;
	}

	fn set_waveform(&mut self, w: synth::Waveform) {
		self.waveform = w;
	}

}

impl synth::Stream for Synth {

	fn data(&mut self, time: f32) -> f32 {

		let mut sound = 0.0;

		for n in &mut self.notes {
			sound += n.sound(self.waveform, time);
		}

		sound *= self.volume;

		if self.buf.len() >= self.buf.capacity() {
			self.buf.pop_front();
		}

		self.buf.push_back(sound);

		return sound;

	}

}

fn main() -> Result<()> {
	return app::run::<Game>();
}

