// wengwengweng

use std::thread;
use cpal::traits::*;
use super::*;

/// The Audio Context. See [mod-level doc](index.html) for usage.
pub struct Audio {
	mixer: Arc<Mutex<Mixer>>,
	user_streams: Arc<Mutex<Vec<Arc<Mutex<dyn Stream>>>>>,
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
		let user_streams: Arc<Mutex<Vec<Arc<Mutex<dyn Stream>>>>> = Arc::new(Mutex::new(vec![]));
		let t_user_streams = Arc::clone(&user_streams);

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

				match data {

					cpal::StreamData::Output { buffer, } => {

						let mut mixer = match t_mixer.lock() {
							Ok(mixer) => mixer,
							Err(err) => {
								elog!("failed to get mixer");
								return;
							}
						};

						let mut user_streams = match t_user_streams.lock() {
							Ok(user_streams) => user_streams,
							Err(err) => {
								elog!("failed to get user streams");
								return;
							}
						};

						let mut next_frame = || {
							let frame = mixer.next() + user_streams
								.iter_mut()
								.fold(Frame::zero(), |frame_acc, stream| {
									return frame_acc + stream
										.lock()
										.map(|mut s| s.next())
										.unwrap_or(Frame::zero());
								});
							return frame.clamp();
						};

						match buffer {
							cpal::UnknownTypeOutputBuffer::U16(mut output) => {
								for d in output.chunks_mut(2) {
									let frame = next_frame();
									d[0] = utils::f32_to_u16(frame.left);
									d[1] = utils::f32_to_u16(frame.right);
								}
							},
							cpal::UnknownTypeOutputBuffer::I16(mut output) => {
								for d in output.chunks_mut(2) {
									let frame = next_frame();
									d[0] = utils::f32_to_i16(frame.left);
									d[1] = utils::f32_to_i16(frame.right);
								}
							},
							cpal::UnknownTypeOutputBuffer::F32(mut output) => {
								for d in output.chunks_mut(2) {
									let frame = next_frame();
									d[0] = frame.left;
									d[1] = frame.right;
								}
							},
						}

					},

					cpal::StreamData::Input { buffer, } => {
						match buffer {
							cpal::UnknownTypeInputBuffer::U16(input) => {
								// ...
							},
							cpal::UnknownTypeInputBuffer::I16(input) => {
								// ...
							},
							cpal::UnknownTypeInputBuffer::F32(input) => {
								// ...
							},
						}
					},

				}

			});

		}).map_err(|_| format!("failed to spawn audio thread"))?;

		return Ok(Self {
			mixer: mixer,
			user_streams: user_streams,
		});

	}

	pub(super) fn mixer(&self) -> &Arc<Mutex<Mixer>> {
		return &self.mixer;
	}

	pub fn stream<S: Stream + Send + 'static>(&mut self, src: Arc<Mutex<S>>) -> Result<()> {
		self.user_streams
			.lock()
			.map_err(|_| format!("failed to lock user stream"))?
			.push(src.clone());
		return Ok(());
	}

}

