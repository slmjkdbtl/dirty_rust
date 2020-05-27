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
			data_type: format.data_type,
		};

		let event_loop = host.event_loop();
		let stream_id = event_loop
			.build_output_stream(&device, &format)
			.map_err(|_| "failed to build audio output stream".to_string())?;

		event_loop
			.play_stream(stream_id)
			.map_err(|_| "failed to start audio stream".to_string())?;

		let mixer = Arc::new(Mutex::new(Mixer::new(format.sample_rate.0)));
		let t_mixer = Arc::clone(&mixer);

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

				let mut mixer = match t_mixer.lock() {
					Ok(mixer) => mixer,
					Err(err) => {
						elog!("failed to get mixer");
						return;
					}
				};

				match data {

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut output) } => {
						for d in output.chunks_mut(2) {
							let frame = mixer.next().unwrap_or_default();
							d[0] = utils::f32_to_u16(frame.left);
							d[1] = utils::f32_to_u16(frame.right);
						}
					},

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut output) } => {
						for d in output.chunks_mut(2) {
							let frame = mixer.next().unwrap_or_default();
							d[0] = utils::f32_to_i16(frame.left);
							d[1] = utils::f32_to_i16(frame.right);
						}
					},

					cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut output) } => {
						for d in output.chunks_mut(2) {
							let frame = mixer.next().unwrap_or_default();
							d[0] = frame.left;
							d[1] = frame.right;
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

	pub fn play<S: Source + Send + 'static>(&mut self, src: Arc<Mutex<S>>) -> Result<()> {

		let mut mixer = self.mixer
			.lock()
			.map_err(|_| "failed to get mixer".to_string())?;

		mixer.add(src)?;

		return Ok(());

	}

}

