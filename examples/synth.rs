// wengwengweng

#![feature(clamp)]

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

use dirty::*;
use synth::BasicSynth;
use synth::Waveform;
use synth::Voice;
use synth::Envelope;
use input::Key;

fn key_to_note(k: Key, o: i32) -> Option<i32> {

	let o = o + 1;

	return match k {
		Key::A => Some(0 + o * 12),
		Key::W => Some(1 + o * 12),
		Key::S => Some(2 + o * 12),
		Key::E => Some(3 + o * 12),
		Key::D => Some(4 + o * 12),
		Key::F => Some(5 + o * 12),
		Key::T => Some(6 + o * 12),
		Key::G => Some(7 + o * 12),
		Key::Y => Some(8 + o * 12),
		Key::H => Some(9 + o * 12),
		Key::U => Some(10 + o * 12),
		Key::J => Some(11 + o * 12),
		Key::K => Some(12 + o * 12),
		Key::O => Some(13 + o * 12),
		Key::L => Some(14 + o * 12),
		Key::P => Some(15 + o * 12),
		Key::Semicolon => Some(16 + o * 12),
		_ => None,
	};

}

struct Game {
	octave: i32,
	waveform: Waveform,
	envelope: Envelope,
	pressed: HashSet<i32>,
	synth: Arc<Mutex<BasicSynth>>,
}

impl Game {

	fn press_note(&mut self, note: i32) {

		self.pressed.insert(note);

		let v = Voice::builder(note)
			.waveform(self.waveform)
			.envelope(self.envelope)
			.build();

		if let Ok(mut synth) = self.synth.lock() {
			synth.play(v);
		}

	}

	fn release_note(&mut self, note: i32) {

		if self.pressed.contains(&note) {
			self.pressed.remove(&note);
			if let Ok(mut synth) = self.synth.lock() {
				synth.release(note);
			}
		}


	}

}

impl State for Game {

	fn init(_: &mut Ctx) -> Result<Self> {

		let synth = Arc::new(Mutex::new(BasicSynth::new()));

		synth::run(synth.clone());

		return Ok(Self {
			octave: 4,
			waveform: Waveform::Saw,
			pressed: hset![],
			synth: synth,
			envelope: Envelope {
				attack: 0.01,
				decay: 0.01,
				sustain: 1.0,
				release: 1.0,
			},
		});
	}

	fn event(&mut self, ctx: &mut Ctx, e: &input::Event) -> Result<()> {

		use input::Event::*;

		match e {

			KeyPress(k) => {

				let mods = ctx.key_mods();

				match *k {
					Key::Esc => ctx.quit(),
					Key::F => {
						if mods.meta {
							ctx.toggle_fullscreen()
						}
					},
					Key::Up => {},
					Key::Down => {},
					Key::Left => self.octave -= 1,
					Key::Right => self.octave += 1,
					Key::Key1 => self.waveform = Waveform::Sine,
					Key::Key2 => self.waveform = Waveform::Triangle,
					Key::Key3 => self.waveform = Waveform::Square,
					Key::Key4 => self.waveform = Waveform::Saw,
					Key::Key5 => self.waveform = Waveform::Noise,
					_ => {},
				}

				if let Some(note) = key_to_note(*k, self.octave) {
					self.press_note(note);
				}

			},

			KeyRelease(k) => {

				if let Some(note) = key_to_note(*k, self.octave) {
					self.release_note(note);
				}

			},

			MIDI(msg) => {

				match msg {
					midi::Msg::NoteOn(note, _) => self.press_note(*note),
					midi::Msg::NoteOff(note, _) => self.release_note(*note),
					_ => {},
				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		if let Ok(synth) = self.synth.lock() {

			let buf = synth.buf();
			let mut last = None;
			let height = 120.0;
			let len = buf.len() as f32;
			let dis = ctx.width() as f32 / len;

			for (i, buf) in buf.iter().enumerate() {

				if let Some(last) = last {

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

fn main() -> Result<()> {
	return run::<Game>();
}

