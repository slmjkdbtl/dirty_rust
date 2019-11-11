// wengwengweng

//! Software Synthesizer

export!(envelope);
export!(life);
export!(wav);

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::collections::VecDeque;
use std::collections::HashMap;

use cpal::traits::*;
use once_cell::sync::Lazy;
use owning_ref::OwningRef;

use crate::*;

const FREQ_A: f32 = 440.0;

pub trait Stream: Send + Sync {
	fn data(&mut self, time: f32) -> f32;
}

#[derive(Clone, Debug)]
pub struct Voice {
	life: Life,
	waveform: Waveform,
	note: NoteOctave,
	volume: f32,
}

impl Voice {

	fn tick(&mut self, dt: f32) {
		self.life.update(dt);
	}

	fn voice(&self, time: f32) -> f32 {
		return self.life.amp() * self.waveform.osc(self.note.to_freq() as f32, time) * self.volume;
	}

	fn dead(&self) -> bool {
		return self.life.dead();
	}

	fn release(&mut self) {
		self.life.release();
	}

}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NoteOctave {
	note: Note,
	octave: i32,
}

impl NoteOctave {

	pub fn new(n: Note, o: i32) -> Self {
		return Self {
			note: n,
			octave: o,
		};
	}

	pub fn to_freq(&self) -> i32 {

		let offset = match self.note {
			Note::C => -9,
			Note::Csh => -8,
			Note::Db => -8,
			Note::D => -7,
			Note::Dsh => -6,
			Note::Eb => -6,
			Note::E => -5,
			Note::F => -4,
			Note::Fsh => -3,
			Note::Gb => -3,
			Note::G => -2,
			Note::Gsh => -1,
			Note::Ab => -1,
			Note::A => 0,
			Note::Ash => 1,
			Note::Bb => 1,
			Note::B => 2,
		};

		let offset = offset + self.octave * 12;

		return (FREQ_A * f32::powi(f32::powf(2.0, 1.0 / 12.0), offset)) as i32;

	}

}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Note {
	C,
	Csh,
	Db,
	D,
	Dsh,
	Eb,
	E,
	F,
	Fsh,
	Gb,
	G,
	Gsh,
	Ab,
	A,
	Ash,
	Bb,
	B,
}

impl Note {

	pub fn to_freq(&self, octave: i32) -> i32 {
		return NoteOctave::new(*self, octave).to_freq();
	}

}

pub struct Synth {
	notes: HashMap<NoteOctave, Voice>,
	volume: f32,
	last_time: f32,
	buf: VecDeque<f32>,
}

impl Synth {

	pub fn new() -> Self {
		return Synth {
			volume: 1.0,
			notes: hashmap![],
			last_time: 0.0,
			buf: VecDeque::with_capacity(100),
		};
	}

	pub fn volume(&self) -> f32 {
		return self.volume;
	}

	pub fn set_volume(&mut self, v: f32) {
		self.volume = v.clamp(0.0, 1.0);
	}

	pub fn play(&mut self, v: Voice) {
		self.notes.insert(v.note, v);
	}

}

impl synth::Stream for Synth {

	fn data(&mut self, time: f32) -> f32 {

		let dt = if time >= self.last_time {
			time - self.last_time
		} else {
			(1.0 + time) - self.last_time
		};

		self.last_time = time;

		let mut sound = 0.0;

		for (_, n) in &mut self.notes {
			sound += n.voice(time);
		}

		sound *= self.volume;

		for (_, n) in &mut self.notes {
			n.tick(dt);
		}

		self.notes.retain(|_, n| !n.dead());

		if self.buf.len() >= self.buf.capacity() {
			self.buf.pop_front();
		}

		self.buf.push_back(sound);

		return sound;

	}

}

static SYNTH: Lazy<Arc<Mutex<Synth>>> = Lazy::new(|| {

	let synth = Synth::new();
	let synth = Arc::new(Mutex::new(synth));

	run(synth.clone());

	return synth;

});

#[derive(Clone, Copy, Debug)]
pub struct VoiceBuilder {
	note: NoteOctave,
	envelope: Envelope,
	waveform: Waveform,
	volume: f32,
}

impl VoiceBuilder {

	pub fn envelope(mut self, e: Envelope) -> Self {
		self.envelope = e;
		return self;
	}

	pub fn attack(mut self, a: f32) -> Self {
		self.envelope.attack = a;
		return self;
	}

	pub fn decay(mut self, d: f32) -> Self {
		self.envelope.decay = d;
		return self;
	}

	pub fn sustain(mut self, s: f32) -> Self {
		self.envelope.sustain = s;
		return self;
	}

	pub fn release(mut self, r: f32) -> Self {
		self.envelope.release = r;
		return self;
	}

	pub fn waveform(mut self, w: Waveform) -> Self {
		self.waveform = w;
		return self;
	}

	pub fn volume(mut self, v: f32) -> Self {
		self.volume = v;
		return self;
	}

	pub fn build(self) -> Voice {

		return Voice {
			volume: self.volume,
			note: self.note,
			waveform: self.waveform,
			life: Life::new(self.envelope),
		};

	}

}

pub fn play(n: Voice) {

	let mut synth = match SYNTH.lock() {
		Ok(s) => s,
		Err(_) => return,
	};

	synth.play(n);

}

pub fn play_oneshot(mut n: Voice) {

	let mut synth = match SYNTH.lock() {
		Ok(s) => s,
		Err(_) => return,
	};

	n.release();
	synth.play(n);

}

pub fn release(note: NoteOctave) {

	let mut synth = match SYNTH.lock() {
		Ok(s) => s,
		Err(_) => return,
	};

	if let Some(n) = synth.notes.get_mut(&note) {
		n.release();
	}

}

pub fn buf() -> Option<OwningRef<MutexGuard<'static, Synth>, VecDeque<f32>>> {

	let synth = match SYNTH.lock() {
		Ok(s) => s,
		Err(_) => return None,
	};

	let synth = OwningRef::new(synth);

	return Some(synth.map(|s| &s.buf));

}

pub fn build_voice(note: NoteOctave) -> VoiceBuilder {
	return VoiceBuilder {
		volume: 1.0,
		note: note,
		waveform: Waveform::Sine,
		envelope: Envelope {
			attack: 0.01,
			decay: 0.01,
			sustain: 1.0,
			release: 1.0,
		},
	};
}

pub fn run(stream: Arc<Mutex<dyn Stream>>) -> Result<()> {

	let host = cpal::default_host();
	let device = host
		.default_output_device()
		.ok_or(Error::Audio(format!("failed to get default output device")))?;
	let format = device.default_output_format()?;
	let event_loop = host.event_loop();
	let stream_id = event_loop.build_output_stream(&device, &format)?;

	event_loop.play_stream(stream_id.clone())?;

	std::thread::spawn(move || {

		let sample_rate = format.sample_rate.0 as f32;
		let mut sample_clock = 0f32;

		let mut tick = || {
			sample_clock = (sample_clock + 1.0) % sample_rate;
			return sample_clock / sample_rate;
		};

		event_loop.run(move |id, data| {

			let data = match data {
				Ok(data) => data,
				Err(err) => {
					eprintln!("an error occurred on stream {:?}: {}", id, err);
					return;
				}
			};

			match data {

				cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {

					for sample in buffer.chunks_mut(format.channels as usize) {

						let dt = tick();

						if let Ok(mut stream) = stream.lock() {

							let value = ((stream.data(dt) * 0.5 + 0.5) * std::u16::MAX as f32) as u16;

							for out in sample.iter_mut() {
								*out = value;
							}

						}

					}

				},

				cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {

					for sample in buffer.chunks_mut(format.channels as usize) {

						let dt = tick();

						if let Ok(mut stream) = stream.lock() {

							let value = (stream.data(dt) * std::i16::MAX as f32) as i16;

							for out in sample.iter_mut() {
								*out = value;
							}

						}

					}

				},

				cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

					for sample in buffer.chunks_mut(format.channels as usize) {

						let dt = tick();

						if let Ok(mut stream) = stream.lock() {

							let value = stream.data(dt);

							for out in sample.iter_mut() {
								*out = value;
							}

						}

					}

				},

				_ => (),

			}

		});

	});

	return Ok(());

}

