// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

use cpal::traits::*;

use super::*;

/// The Audio Context. See [mod-level doc](super) for usage.
pub struct Audio {
	mixer: Arc<Mutex<Mixer>>,
}

impl Audio {

	pub(crate) fn new() -> Result<Self> {

		let mixer = Arc::new(Mutex::new(Mixer::new()));
		let t_mixer = Arc::clone(&mixer);

		let host = cpal::default_host();

		let device = host
			.default_output_device()
			.ok_or("failed to get default output device".to_string())?;

		let format = device
			.default_output_format()
			.map_err(|_| "failed to get default audio output format".to_string())?;

		let format = cpal::Format {
			channels: CHANNEL_COUNT.to_cpal(),
			sample_rate: cpal::SampleRate(SAMPLE_RATE),
			data_type: cpal::SampleFormat::F32,
		};

		let event_loop = host.event_loop();
		let stream_id = event_loop
			.build_output_stream(&device, &format)
			.map_err(|_| "failed to build audio output stream".to_string())?;

		event_loop
			.play_stream(stream_id)
			.map_err(|_| "failed to start audio stream".to_string())?;

		thread::Builder::new()
			.name(String::from("dirty_audio"))
			.spawn(move || {

			event_loop.run(move |id, data| {

				let data = match data {
					Ok(data) => data,
					Err(err) => {
						elog!("an error occurred on stream {:?}: {}", id, err);
						return;
					}
				};

				match data {

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut output) } => {
						if let Ok(mut mixer) = t_mixer.lock() {
							for d in output.chunks_mut(2) {
								if let Some(frame) = mixer.next() {
									d[0] = utils::f32_to_u16(frame.left);
									d[1] = utils::f32_to_u16(frame.right);
								}
							}
						}
					},

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut output) } => {
						if let Ok(mut mixer) = t_mixer.lock() {
							for d in output.chunks_mut(2) {
								if let Some(frame) = mixer.next() {
									d[0] = utils::f32_to_i16(frame.left);
									d[1] = utils::f32_to_i16(frame.right);
								}
							}
						}
					},

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut output) } => {
						if let Ok(mut mixer) = t_mixer.lock() {
							for d in output.chunks_mut(2) {
								if let Some(frame) = mixer.next() {
									d[0] = frame.left;
									d[1] = frame.right;
								}
							}
						}
					},

					_ => (),

				}

			});

		}).map_err(|_| "failed to spawn audio thread".to_string())?;

		return Ok(Self {
			mixer,
		});

	}

	pub(super) fn mixer(&self) -> &Arc<Mutex<Mixer>> {
		return &self.mixer;
	}

	pub fn sample_rate(&self) -> u32 {
		return SAMPLE_RATE;
	}

	pub fn play<S: Source + Send + 'static>(&mut self, src: Arc<Mutex<S>>) {
		if let Ok(mut mixer) = self.mixer.lock() {
			mixer.add(src);
		}
	}

}

