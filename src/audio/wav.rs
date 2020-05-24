// wengwengweng

use std::time::Duration;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use super::*;

pub struct WavDecoder<R: Read + Seek> {
	decoder: hound::WavReader<R>,
	specs: hound::WavSpec,
	duration: Duration,
	channel_count: ChannelCount,
}

impl<R: Read + Seek> WavDecoder<R> {

	pub fn new(data: R) -> Result<Self> {

		let wav = hound::WavReader::new(data).map_err(|_| format!("failed to parse wav"))?;
		let spec = wav.spec();

		let channel_count = match spec.channels {
			1 => ChannelCount::One,
			2 => ChannelCount::Two,
			_ => return Err(format!("unsupported channel count: {}", spec.channels)),
		};

		let ms = wav.len() as usize * 1000 / (spec.channels as usize * spec.sample_rate as usize);
		let duration = Duration::from_millis(ms as u64);

		return Ok(Self {
			specs: spec,
			decoder: wav,
			duration: duration,
			channel_count: channel_count,
		});

	}

	fn next_sample(&mut self) -> Option<f32> {

		use hound::SampleFormat::*;

		return match (self.specs.sample_format, self.specs.bits_per_sample) {
			(Float, 32) => self.decoder.samples::<f32>().next().map(|sample| {
				return sample.unwrap_or(0.0);
			}),
			(Int, 16) => self.decoder.samples::<i16>().next().map(|sample| {
				return utils::i16_to_f32(sample.unwrap_or(0));
			}),
			_ => None,
		};

	}

}

impl<R: Read + Seek> Source for WavDecoder<R> {}

impl<R: Read + Seek> Iterator for WavDecoder<R> {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		let sample = match self.next_sample() {
			Some(sample) => sample,
			None => return None,
		};

		return Some(match self.channel_count {
			ChannelCount::One => (sample, sample),
			ChannelCount::Two => (sample, self.next_sample().unwrap_or(0.0)),
		});

	}

}

// TODO
pub fn is_wav<R: Read + Seek>(mut data: R) -> bool {

	let pos = match data.seek(SeekFrom::Current(0)) {
		Ok(pos) => pos,
		Err(_) => return false,
	};

	if hound::WavReader::new(data.by_ref()).is_err() {
		data.seek(SeekFrom::Start(pos));
		return false;
	}

	data.seek(SeekFrom::Start(pos));

	return true;

}

