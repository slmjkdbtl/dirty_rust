// wengwengweng

export!(envelope);
export!(note);
export!(waveforms);

use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::Mutex;

use cpal::traits::*;

use crate::*;

const FREQ_A: f32 = 440.0;

pub fn osc(wav: Waveform, freq: f32, dt: f32) -> f32 {
	return wav.osc(freq, dt);
}

pub fn get_note_freq(t: i32) -> f32 {
	return FREQ_A * f32::powi(f32::powf(2.0, 1.0 / 12.0), t);
}

pub trait Stream: Send + Sync {
	fn data(&self, dt: f32) -> f32;
}

pub fn run(synth: Arc<Mutex<dyn Stream>>) -> Result<()> {

	let host = cpal::default_host();
	let device = host.default_output_device().expect("failed to find a default output device");
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
						let value = ((synth.lock().unwrap().data(dt) * 0.5 + 0.5) * std::u16::MAX as f32) as u16;

						for out in sample.iter_mut() {
							*out = value;
						}

					}

				},

				cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {

					for sample in buffer.chunks_mut(format.channels as usize) {

						let dt = tick();
						let value = (synth.lock().unwrap().data(dt) * std::i16::MAX as f32) as i16;

						for out in sample.iter_mut() {
							*out = value;
						}

					}

				},

				cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

					for sample in buffer.chunks_mut(format.channels as usize) {

						let dt = tick();
						let value = synth.lock().unwrap().data(dt);

						for out in sample.iter_mut() {
							*out = value;
						}

					}

				},

				_ => (),

			}

		});

	});

	return Ok(());

}


