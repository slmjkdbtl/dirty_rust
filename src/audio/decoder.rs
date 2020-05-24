// wengwengweng

use std::io::Read;
use std::io::Seek;

use super::*;

pub(super) enum Decoder<R: Read + Seek> {
	Wav(WavDecoder<R>),
	Mp3(Mp3Decoder<R>),
	Vorbis(VorbisDecoder<R>),
}

impl<R: Read + Seek> Decoder<R> {

	pub fn new(mut reader: R) -> Result<Self> {

		if is_vorbis(reader.by_ref()) {
			return Ok(Self::Vorbis(VorbisDecoder::new(reader)?));
		}

		if is_wav(reader.by_ref()) {
			return Ok(Self::Wav(WavDecoder::new(reader)?));
		}

		if is_mp3(reader.by_ref()) {
			return Ok(Self::Mp3(Mp3Decoder::new(reader)?));
		}

		return Err(format!("failed to decode audio"));

	}
}

impl<R: Read + Seek> Source for Decoder<R> {
	fn sample_rate(&self) -> SampleRate {
		return match self {
			Decoder::Wav(decoder) => decoder.sample_rate(),
			Decoder::Mp3(decoder) => decoder.sample_rate(),
			Decoder::Vorbis(decoder) => decoder.sample_rate(),
		};
	}
}

impl<R: Read + Seek> Iterator for Decoder<R> {
	type Item = Frame;
	fn next(&mut self) -> Option<Self::Item> {
		return match self {
			Decoder::Wav(decoder) => decoder.next(),
			Decoder::Mp3(decoder) => decoder.next(),
			Decoder::Vorbis(decoder) => decoder.next(),
		};
	}
}

