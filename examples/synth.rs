// wengwengweng

#![feature(clamp)]

use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use dirty::*;
use app::*;
use synth::*;
use input::Key;

fn key_to_note(k: Key, o: i32) -> Option<NoteOctave> {

	return match k {
		Key::A => Some(NoteOctave::new(Note::C, o)),
		Key::W => Some(NoteOctave::new(Note::Csh, o)),
		Key::S => Some(NoteOctave::new(Note::D, o)),
		Key::E => Some(NoteOctave::new(Note::Dsh, o)),
		Key::D => Some(NoteOctave::new(Note::E, o)),
		Key::F => Some(NoteOctave::new(Note::F, o)),
		Key::T => Some(NoteOctave::new(Note::Fsh, o)),
		Key::G => Some(NoteOctave::new(Note::G, o)),
		Key::Y => Some(NoteOctave::new(Note::Gsh, o)),
		Key::H => Some(NoteOctave::new(Note::A, o)),
		Key::U => Some(NoteOctave::new(Note::Ash, o)),
		Key::J => Some(NoteOctave::new(Note::B, o)),
		Key::K => Some(NoteOctave::new(Note::C, o + 1)),
		Key::O => Some(NoteOctave::new(Note::Csh, o + 1)),
		Key::L => Some(NoteOctave::new(Note::D, o + 1)),
		Key::P => Some(NoteOctave::new(Note::Dsh, o + 1)),
		Key::Semicolon => Some(NoteOctave::new(Note::E, o + 1)),
		_ => None,
	};

}

struct Game {
	octave: i32,
	waveform: Waveform,
	envelope: Envelope,
	pressed: HashMap<Key, NoteOctave>,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			octave: -1,
			waveform: Waveform::Saw,
			pressed: hashmap![],
			envelope: Envelope {
				attack: 0.01,
				decay: 0.01,
				sustain: 1.0,
				release: 1.0,
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

				if k == Key::Right {
					self.octave += 1;
				}

				if k == Key::Left {
					self.octave -= 1;
				}

				if k == Key::Up {
					// ...
				}

				if k == Key::Down {
					// ...
				}

				if k == Key::Key1 {
					self.waveform = Waveform::Sine;
				}

				if k == Key::Key2 {
					self.waveform = Waveform::Triangle;
				}

				if k == Key::Key3 {
					self.waveform = Waveform::Square;
				}

				if k == Key::Key4 {
					self.waveform = Waveform::Saw;
				}

				if k == Key::Key5 {
					self.waveform = Waveform::Noise;
				}

				if let Some(note) = key_to_note(k, self.octave) {

					self.pressed.insert(k, note);

					let v = build_voice(note)
						.waveform(self.waveform)
						.build();

					play(v);

				}

			},

			KeyRelease(k) => {

				if let Some(n) = self.pressed.get(&k) {
					synth::release(*n);
				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut app::Ctx) -> Result<()> {

		if let Some(buf) = synth::buf() {

			let mut last = None;
			let height = 120.0;
			let len = buf.len() as f32;
			let dis = ctx.gwidth() as f32 / len;

			for (i, buf) in buf.iter().enumerate() {

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

fn main() -> Result<()> {
	return app::run::<Game>();
}

