// wengwengweng

use std::time::Duration;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use super::*;

pub struct WavDecoder<R: Read + Seek> {
	decoder: hound::WavReader<R>,
	spec: hound::WavSpec,
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

		let duration = Duration::from_secs_f32(wav.duration() as f32 / spec.sample_rate as f32);

		return Ok(Self {
			spec: spec,
			decoder: wav,
			duration: duration,
			channel_count: channel_count,
		});

	}

	fn next_sample(&mut self) -> Option<f32> {

		use hound::SampleFormat::*;

		return match (self.spec.sample_format, self.spec.bits_per_sample) {
			(Float, 32) => self.decoder.samples::<f32>().next().map(|sample| {
				return sample.unwrap_or(0.0);
			}),
			(Int, 16) => self.decoder.samples::<i16>().next().map(|sample| {
				return utils::i16_to_f32(sample.unwrap_or(0));
			}),
			_ => None,
		};

	}

	pub fn reset(&mut self) -> Result<()> {
		self.decoder
			.seek(0)
			.map_err(|_| format!("failed to seek wav"))?;
		return Ok(());
	}

}

impl<R: Read + Seek> Source for WavDecoder<R> {
	fn sample_rate(&self) -> u32 {
		return self.spec.sample_rate;
	}
}

impl<R: Read + Seek> Iterator for WavDecoder<R> {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		let sample = match self.next_sample() {
			Some(sample) => sample,
			None => return None,
		};

		return Some(match self.channel_count {
			ChannelCount::One => Frame::new(sample, sample),
			ChannelCount::Two => Frame::new(sample, self.next_sample().unwrap_or(0.0)),
		});

	}

}

pub fn is_wav<R: Read + Seek>(mut reader: R) -> Result<bool> {

	let pos = reader
		.seek(SeekFrom::Current(0))
		.map_err(|_| format!("failed to seek"))?;

	let is_wav = hound::WavReader::new(&mut reader).is_ok();

	reader
		.seek(SeekFrom::Start(pos))
		.map_err(|_| format!("failed to seek"))?;

	return Ok(is_wav);

}

