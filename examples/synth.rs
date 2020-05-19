// wengwengweng

#![feature(clamp)]

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

use dirty::*;
use gfx::shapes;
use synth::BasicSynth;
use synth::Waveform;
use synth::Voice;
use synth::Envelope;
use synth::Note;
use input::Key;

fn key_to_note(k: Key, o: i32) -> Option<Note> {

	let n = match k {
		Key::A => Some(0),
		Key::W => Some(1),
		Key::S => Some(2),
		Key::E => Some(3),
		Key::D => Some(4),
		Key::F => Some(5),
		Key::T => Some(6),
		Key::G => Some(7),
		Key::Y => Some(8),
		Key::H => Some(9),
		Key::U => Some(10),
		Key::J => Some(11),
		Key::K => Some(12),
		Key::O => Some(13),
		Key::L => Some(14),
		Key::P => Some(15),
		Key::Semicolon => Some(16),
		_ => None,
	};

	return n.map(|n| Note::from_octave(n, o));

}

struct Game {
	octave: i32,
	waveform: Waveform,
	envelope: Envelope,
	pressed: HashSet<Note>,
	synth: Arc<Mutex<BasicSynth>>,
}

impl Game {

	fn press_note(&mut self, note: Note) {

		self.pressed.insert(note);

		let v = Voice::builder(note)
			.waveform(self.waveform)
			.envelope(self.envelope)
			.build();

		if let Ok(mut synth) = self.synth.lock() {
			synth.play(v);
		}

	}

	fn release_note(&mut self, note: Note) {

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
			waveform: Waveform::Triangle,
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

		let win = &mut ctx.window;

		match e {

			KeyPress(k) => {

				let mods = win.key_mods();

				match *k {
					Key::Esc => win.quit(),
					Key::F => {
						if mods.meta {
							win.toggle_fullscreen()
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
					midi::Msg::NoteOn(n, _) => self.press_note(Note::new(*n)),
					midi::Msg::NoteOff(n, _) => self.release_note(Note::new(*n)),
					_ => {},
				}

			},

			_ => {},

		}

		return Ok(());

	}

	fn draw(&mut self, ctx: &mut Ctx) -> Result<()> {

		let gfx = &mut ctx.gfx;

		if let Ok(synth) = self.synth.lock() {

			let buf = synth.buf();
			let mut last = None;
			let height = 120.0;
			let len = buf.len() as f32;
			let dis = gfx.width() as f32 / len;

			for (i, buf) in buf.iter().enumerate() {

				if let Some(last) = last {

					let ay = last * height;
					let by = buf * height;
					let ax = -len / 2.0 * dis + (i - 1) as f32 * dis;
					let bx = -len / 2.0 * dis + i as f32 * dis;

					gfx.draw(&shapes::line(vec2!(ax, ay), vec2!(bx, by)))?;

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

