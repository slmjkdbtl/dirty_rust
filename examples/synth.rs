// wengwengweng

#![feature(clamp)]

use std::collections::HashMap;

use dirty::*;
use app::*;
use synth::*;
use input::Key;

fn key_to_note(k: Key, o: i32) -> Option<Note> {

	return match k {
		Key::A => Some(Note::C(o)),
		Key::W => Some(Note::Csh(o)),
		Key::S => Some(Note::D(o)),
		Key::E => Some(Note::Dsh(o)),
		Key::D => Some(Note::E(o)),
		Key::F => Some(Note::F(o)),
		Key::T => Some(Note::Fsh(o)),
		Key::G => Some(Note::G(o)),
		Key::Y => Some(Note::Gsh(o)),
		Key::H => Some(Note::A(o)),
		Key::U => Some(Note::Ash(o)),
		Key::J => Some(Note::B(o)),
		Key::K => Some(Note::C(o + 1)),
		Key::O => Some(Note::Csh(o + 1)),
		Key::L => Some(Note::D(o + 1)),
		Key::P => Some(Note::Dsh(o + 1)),
		Key::Semicolon => Some(Note::E(o + 1)),
		_ => None,
	};

}

struct Game {
	octave: i32,
	waveform: Waveform,
	envelope: Envelope,
	pressed: HashMap<Key, Note>,
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
						.envelope(self.envelope)
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

