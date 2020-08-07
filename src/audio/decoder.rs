// wengwengweng

use super::*;

pub(super) enum Decoder<R: Read + Seek> {
	Wav(WavPlayback<R>),
	Mp3(Mp3Playback<R>),
	Vorbis(VorbisPlayback<R>),
}

impl<R: Read + Seek> Decoder<R> {

	pub fn new(mut reader: R) -> Result<Self> {

		if is_vorbis(&mut reader)? {
			return Ok(Self::Vorbis(VorbisPlayback::new(reader)?));
		}

		if is_wav(&mut reader)? {
			return Ok(Self::Wav(WavPlayback::new(reader)?));
		}

		if is_mp3(&mut reader)? {
			return Ok(Self::Mp3(Mp3Playback::new(reader)?));
		}

		return Err(format!("failed to decode audio"));

	}

}

impl<R: Read + Seek> Source for Decoder<R> {
	fn sample_rate(&self) -> u32 {
		return match self {
			Decoder::Wav(decoder) => decoder.sample_rate(),
			Decoder::Mp3(decoder) => decoder.sample_rate(),
			Decoder::Vorbis(decoder) => decoder.sample_rate(),
		};
	}
	fn seek_start(&mut self) -> Result<()> {
		return match self {
			Decoder::Wav(decoder) => decoder.seek_start(),
			Decoder::Mp3(decoder) => decoder.seek_start(),
			Decoder::Vorbis(decoder) => decoder.seek_start(),
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

