// wengwengweng

//! Software Synthesizer

export!(envelope);
export!(life);
export!(wav);
export!(note);
export!(voice);
export!(scale);

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::collections::VecDeque;
use std::collections::HashMap;

use cpal::traits::*;
use once_cell::sync::Lazy;
use owning_ref::OwningRef;

use crate::*;

pub trait Stream: Send + Sync {
	fn data(&mut self, time: f32) -> f32;
}

pub struct Synth {
	notes: HashMap<NoteO, Voice>,
	volume: f32,
	last_time: f32,
	buf: VecDeque<f32>,
}

impl Synth {

	pub fn new() -> Self {
		return Synth {
			volume: 1.0,
			notes: hmap![],
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

	pub fn play_oneshot(&mut self, v: Voice) {

		let n = v.note;

		self.notes.insert(n, v);
		self.release(n);

	}

	pub fn release(&mut self, n: NoteO) {

		if let Some(n) = self.notes.get_mut(&n) {
			n.release();
		}

	}

}

impl Stream for Synth {

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

pub fn play(n: Voice) {

	let mut synth = match SYNTH.lock() {
		Ok(s) => s,
		Err(_) => return,
	};

	synth.play(n);

}

pub fn play_oneshot(n: Voice) {

	let mut synth = match SYNTH.lock() {
		Ok(s) => s,
		Err(_) => return,
	};

	synth.play_oneshot(n);

}

pub fn release(note: NoteO) {

	let mut synth = match SYNTH.lock() {
		Ok(s) => s,
		Err(_) => return,
	};

	synth.release(note);

}

pub fn buf() -> Option<OwningRef<MutexGuard<'static, Synth>, VecDeque<f32>>> {

	let synth = match SYNTH.lock() {
		Ok(s) => s,
		Err(_) => return None,
	};

	let synth = OwningRef::new(synth);

	return Some(synth.map(|s| &s.buf));

}

pub fn build_voice(note: NoteO) -> VoiceBuilder {
	return Voice::builder(note);
}

fn run(stream: Arc<Mutex<dyn Stream>>) -> Result<()> {

	let host = cpal::default_host();

	let device = host
		.default_output_device()
		.ok_or(format!("failed to get default output device"))?;

	let format = device
		.default_output_format()
		.map_err(|_| format!("failed to get default audio output format"))?;

	let event_loop = host.event_loop();
	let stream_id = event_loop
		.build_output_stream(&device, &format)
		.map_err(|_| format!("failed to build audio output stream"))?;

	event_loop
		.play_stream(stream_id.clone())
		.map_err(|_| format!("failed to start audio stream"))?;

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

