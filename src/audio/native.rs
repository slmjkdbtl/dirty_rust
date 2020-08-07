// wengwengweng

use std::thread;
use cpal::traits::*;
use super::*;

/// The Audio Context. See [mod-level doc](index.html) for usage.
pub struct Audio {
	mixer: Arc<Mutex<Mixer>>,
}

impl Audio {

	pub(crate) fn new(_: &conf::Conf) -> Result<Self> {

		let host = cpal::default_host();

		let device = host
			.default_output_device()
			.ok_or(format!("failed to get default output device"))?;

		let format = device
			.default_output_format()
			.map_err(|_| format!("failed to get default audio output format"))?;

		let format = cpal::Format {
			channels: SPEC.channel_count,
			sample_rate: cpal::SampleRate(SPEC.sample_rate),
			data_type: format.data_type,
		};

		let event_loop = host.event_loop();
		let stream_id = event_loop
			.build_output_stream(&device, &format)
			.map_err(|_| format!("failed to build audio output stream"))?;

		event_loop
			.play_stream(stream_id)
			.map_err(|_| format!("failed to start audio stream"))?;

		let mixer = Arc::new(Mutex::new(Mixer::new(SPEC)));
		let t_mixer = Arc::clone(&mixer);

		thread::Builder::new()
			.name(format!("dirty_audio"))
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

		}).map_err(|_| format!("failed to spawn audio thread"))?;

		return Ok(Self {
			mixer,
		});

	}

	pub(super) fn mixer(&self) -> &Arc<Mutex<Mixer>> {
		return &self.mixer;
	}

	pub fn play<S: Source + Send + 'static>(&mut self, src: Arc<Mutex<S>>) -> Result<Arc<Mutex<Control>>> {
		return Ok(self.mixer
			.lock()
			.map_err(|_| format!("failed to get mixer"))?
			.add(src));
	}

}

