// wengwengweng

use std::time::Duration;
use std::io::Read;
use std::io::Seek;

use super::*;

pub struct WavDecoder<R: Read + Seek> {
	decoder: hound::WavReader<R>,
	specs: hound::WavSpec,
	duration: Duration,
	paused: bool,
}

impl<R: Read + Seek> WavDecoder<R> {

	pub fn from_reader(data: R) -> Result<Self> {

		let wav = hound::WavReader::new(data).map_err(|_| format!("failed to read wav"))?;
		let spec = wav.spec();

		let ms = wav.len() * 1000 / (spec.channels as u32 * spec.sample_rate as u32);
		let len = Duration::from_millis(ms as u64);

		dbg!(&spec);

		return Ok(Self {
			specs: spec,
			decoder: wav,
			duration: len,
			paused: false,
		});

	}

}

impl<R: Read + Seek> Source for WavDecoder<R> {
// 	fn duration(&self) -> Duration {
// 		return self.duration;
// 	}
// 	fn sample_rate(&self) -> u32 {
// 		return self.specs.sample_rate;
// 	}
// 	fn channels(&self) -> u16 {
// 		return self.specs.channels;
// 	}
}

impl<R: Read + Seek> Iterator for WavDecoder<R> {

	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {

		use hound::SampleFormat::*;

		let v = match (self.specs.sample_format, self.specs.bits_per_sample) {
			(Float, 32) => self.decoder.samples::<f32>().next().map(|value| {
				return value.unwrap_or(0.0);
			}),
			(Int, 16) => self.decoder.samples::<i16>().next().map(|value| {
				return i16_to_f32(value.unwrap_or(0));
			}),
			_ => None,
		};

		return v;

	}

}

