// wengwengweng

//! Software Synthesizer

export!(note);
export!(envelope);
export!(life);
export!(wav);
export!(voice);
export!(basic);

pub mod scale;
pub mod chord;

use std::sync::Arc;
use std::sync::Mutex;

use cpal::traits::*;

use crate::*;

// TODO: a way to stop

pub trait Stream: Send + Sync {
	fn data(&mut self, time: f32) -> f32;
}

pub fn run(stream: Arc<Mutex<dyn Stream>>) -> Result<()> {

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

							let value = ((stream.data(dt) * 0.5 + 0.5) * u16::MAX as f32) as u16;

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

							let value = (stream.data(dt) * i16::MAX as f32) as i16;

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

