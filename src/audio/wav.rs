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
}

impl<R: Read + Seek> WavDecoder<R> {

	pub fn new(data: R) -> Result<Self> {

		let wav = hound::WavReader::new(data).map_err(|_| format!("failed to parse wav"))?;
		let spec = wav.spec();

		let ms = wav.len() as usize * 1000 / (spec.channels as usize * spec.sample_rate as usize);
		let len = Duration::from_millis(ms as u64);

		return Ok(Self {
			specs: spec,
			decoder: wav,
			duration: len,
		});

	}

}

impl<R: Read + Seek> Source for WavDecoder<R> {}

impl<R: Read + Seek> Iterator for WavDecoder<R> {

	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {

		use hound::SampleFormat::*;

		let v = match (self.specs.sample_format, self.specs.bits_per_sample) {
			(Float, 32) => self.decoder.samples::<f32>().next().map(|value| {
				return value.unwrap_or(0.0);
			}),
			(Int, 16) => self.decoder.samples::<i16>().next().map(|value| {
				return utils::i16_to_f32(value.unwrap_or(0));
			}),
			_ => None,
		};

		return v;

	}

}

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

