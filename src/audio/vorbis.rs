// wengwengweng

use std::time::Duration;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::vec;

use lewton::inside_ogg::OggStreamReader;

use super::*;

pub struct VorbisDecoder<R: Read + Seek> {
	reader: OggStreamReader<R>,
	current_data: vec::IntoIter<i16>,
}

impl<R: Read + Seek> Source for VorbisDecoder<R> {}

impl<R: Read + Seek> VorbisDecoder<R> {

	pub fn new(data: R) -> Result<Self> {

		let mut reader = OggStreamReader::new(data).map_err(|_| format!("failed to parse vorbis"))?;

		let mut data = match reader.read_dec_packet_itl().ok().and_then(|v| v) {
			Some(d) => d,
			None => Vec::new(),
		};

		match reader.read_dec_packet_itl().ok().and_then(|v| v) {
			Some(mut d) => data.append(&mut d),
			None => (),
		};

		return Ok(Self {
			reader: reader,
			current_data: data.into_iter(),
		});

	}

}

impl<R: Read + Seek> Iterator for VorbisDecoder<R> {

	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {

		if let Some(sample) = self.current_data.next() {
			if self.current_data.len() == 0 {
				if let Some(data) = self
					.reader
					.read_dec_packet_itl()
					.ok()
					.and_then(|v| v)
				{
					self.current_data = data.into_iter();
				}
			}
			return Some(utils::i16_to_f32(sample));
		} else {
			if let Some(data) = self
				.reader
				.read_dec_packet_itl()
				.ok()
				.and_then(|v| v)
			{
				self.current_data = data.into_iter();
			}
			return self.current_data.next().map(utils::i16_to_f32);
		}

	}

}

pub fn is_vorbis<R: Read + Seek>(mut data: R) -> bool {

	let pos = match data.seek(SeekFrom::Current(0)) {
		Ok(pos) => pos,
		Err(_) => return false,
	};

	if OggStreamReader::new(data.by_ref()).is_err() {
		data.seek(SeekFrom::Start(pos));
		return false;
	}

	data.seek(SeekFrom::Start(pos));

	return true;

}

