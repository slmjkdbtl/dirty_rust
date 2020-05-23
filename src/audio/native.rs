// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

use cpal::traits::HostTrait;
use cpal::traits::DeviceTrait;
use cpal::traits::EventLoopTrait;

use crate::*;
use super::*;

/// The Audio Context. See [mod-level doc](audio) for usage.
pub struct Audio {
	mixer: Arc<Mutex<Mixer>>,
	format: cpal::Format,
}

impl Audio {

	pub(crate) fn new() -> Result<Self> {

		let mixer = Arc::new(Mutex::new(Mixer::new()));
		let t_mixer = Arc::clone(&mixer);

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

		thread::spawn(move || {

			event_loop.run(move |id, data| {

				let data = match data {
					Ok(data) => data,
					Err(err) => {
						elog!("an error occurred on stream {:?}: {}", id, err);
						return;
					}
				};

				match data {

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
						if let Ok(mut mixer) = t_mixer.lock() {
							for d in buffer.iter_mut() {
								*d = mixer.next().map(f32_to_u16).unwrap_or(0);
							}
						}
					},

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
						if let Ok(mut mixer) = t_mixer.lock() {
							for d in buffer.iter_mut() {
								*d = mixer.next().map(f32_to_i16).unwrap_or(0);
							}
						}
					},

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
						if let Ok(mut mixer) = t_mixer.lock() {
							for d in buffer.iter_mut() {
								*d = mixer.next().unwrap_or(0.0);
							}
						}
					},

					_ => (),

				}

			});

		});

		return Ok(Self {
			mixer: mixer,
			format: format,
		});

	}

	pub(super) fn mixer(&self) -> &Arc<Mutex<Mixer>> {
		return &self.mixer;
	}

	pub fn sample_rate(&self) -> f32 {
		return self.format.sample_rate.0 as f32;
	}

	pub fn run<S: Source + Send + 'static>(&mut self, src: Arc<Mutex<S>>) {
		if let Ok(mut mixer) = self.mixer.lock() {
			mixer.add(src);
		}
	}

}

