// wengwengweng

use cpal::traits::*;
use super::*;

/// The Audio Context. See [mod-level doc](index.html) for usage.
pub struct Audio {
	mixer: Arc<Mutex<Mixer>>,
}

fn run<T: cpal::Sample>(device: &cpal::Device, config: &cpal::StreamConfig, mixer: Arc<Mutex<Mixer>>) -> Result<()> {

	let sample_rate = config.sample_rate.0 as f32;
	let channels = config.channels as usize;

	let stream = device.build_output_stream(
		config,
		move |output: &mut [T], _| {

			println!("123");

			let mut mixer = match mixer.lock() {
				Ok(mixer) => mixer,
				Err(err) => {
					elog!("failed to get mixer");
					return;
				}
			};

			for d in output.chunks_mut(SPEC.channel_count as usize) {
				let frame = mixer.next().unwrap_or_default();
				d[0] = cpal::Sample::from::<f32>(&frame.left);
				d[1] = cpal::Sample::from::<f32>(&frame.right);
			}

		},
		|err| elog!("an error occurred on stream: {}", err),
	)
		.map_err(|_| format!("failed to build output stream"))?;

	stream
		.play()
		.map_err(|_| format!("failed to play audio stream"))?;

// 	std::thread::sleep(std::time::Duration::from_millis(100));

	return Ok(());

}

impl Audio {

	pub(crate) fn new(_: &conf::Conf) -> Result<Self> {

		let host = cpal::default_host();

		let device = host
			.default_output_device()
			.ok_or_else(|| format!("failed to get default output device"))?;

		let format = device
			.default_output_config()
			.map_err(|_| format!("failed to get default output config"))?
			.sample_format();

		let config = cpal::StreamConfig {
			channels: SPEC.channel_count,
			sample_rate: cpal::SampleRate(SPEC.sample_rate),
			buffer_size: cpal::BufferSize::Default,
		};

		let mixer = Arc::new(Mutex::new(Mixer::new(SPEC)));

		match format {
			cpal::SampleFormat::F32 => run::<f32>(&device, &config, mixer.clone())?,
			cpal::SampleFormat::I16 => run::<i16>(&device, &config, mixer.clone())?,
			cpal::SampleFormat::U16 => run::<u16>(&device, &config, mixer.clone())?,
		}

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

