// wengwengweng

#![feature(clamp)]

use std::collections::HashMap;

use dirty::*;
use app::*;
use synth::*;
use input::Key;

fn key_to_note(k: Key, o: i32) -> Option<NoteO> {

	return match k {
		Key::A => Some((Note::C, o).into()),
		Key::W => Some((Note::Csh, o).into()),
		Key::S => Some((Note::D, o).into()),
		Key::E => Some((Note::Dsh, o).into()),
		Key::D => Some((Note::E, o).into()),
		Key::F => Some((Note::F, o).into()),
		Key::T => Some((Note::Fsh, o).into()),
		Key::G => Some((Note::G, o).into()),
		Key::Y => Some((Note::Gsh, o).into()),
		Key::H => Some((Note::A, o).into()),
		Key::U => Some((Note::Ash, o).into()),
		Key::J => Some((Note::B, o).into()),
		Key::K => Some((Note::C, o + 1).into()),
		Key::O => Some((Note::Csh, o + 1).into()),
		Key::L => Some((Note::D, o + 1).into()),
		Key::P => Some((Note::Dsh, o + 1).into()),
		Key::Semicolon => Some((Note::E, o + 1).into()),
		_ => None,
	};

}

struct Game {
	octave: i32,
	waveform: Waveform,
	envelope: Envelope,
	pressed: HashMap<Key, NoteO>,
}

impl app::State for Game {

	fn init(_: &mut app::Ctx) -> Result<Self> {

		return Ok(Self {
			octave: -1,
			waveform: Waveform::Saw,
			pressed: hmap![],
			envelope: Envelope {
				attack: 0.01,
				decay: 0.01,
				sustain: 1.0,
				release: 1.0,
			},
		});
	}

	fn event(&mut self, ctx: &mut app::Ctx, e: &input::Event) -> Result<()> {

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

					self.pressed.insert(*k, note);

					let v = build_voice(note)
						.waveform(self.waveform)
						.envelope(self.envelope)
						.build();

					play(v);

				}

			},

			KeyRelease(k) => {

				if let Some(n) = self.pressed.get(k) {
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

