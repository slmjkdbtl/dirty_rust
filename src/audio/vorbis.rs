// wengwengweng

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::vec;

use lewton::inside_ogg::OggStreamReader;

use super::*;

pub struct VorbisDecoder<R: Read + Seek> {
	reader: OggStreamReader<R>,
	cur_packet: Option<vec::IntoIter<i16>>,
	cur_channel: Channel,
	channel_count: ChannelCount,
	last_sample: f32,
}

impl<R: Read + Seek> Source for VorbisDecoder<R> {}

impl<R: Read + Seek> VorbisDecoder<R> {

	pub fn new(data: R) -> Result<Self> {

		let mut reader = OggStreamReader::new(data)
			.map_err(|_| format!("failed to parse vorbis"))?;

		let header = &reader.ident_hdr;

		let channel_count = match header.audio_channels {
			1 => ChannelCount::One,
			2 => ChannelCount::Two,
			_ => return Err(format!("unsupported channel count: {}", header.audio_channels)),
		};

		let data = match reader.read_dec_packet_itl() {
			Ok(data) => data,
			Err(e) => return Err(format!("failed to read vorbis")),
		};

		return Ok(Self {
			reader: reader,
			cur_packet: data.map(|d| d.into_iter()),
			cur_channel: Channel::Left,
			channel_count: channel_count,
			last_sample: 0.0,
		});

	}

}

impl<R: Read + Seek> Iterator for VorbisDecoder<R> {

	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {

		let cur_packet = match &mut self.cur_packet {
			Some(packet) => packet,
			None => return None,
		};

		match self.channel_count {
			ChannelCount::One => {
				match self.cur_channel {
					Channel::Left => self.cur_channel = Channel::Right,
					Channel::Right => {
						self.cur_channel = Channel::Left;
						return Some(self.last_sample);
					}
				}
			},
			_ => {},
		}

		if let Some(sample) = cur_packet.next() {

			let sample = utils::i16_to_f32(sample);

			self.last_sample = sample;

			return Some(sample);

		} else {

			self.cur_packet = self.reader
				.read_dec_packet_itl()
				.ok()
				.flatten()
				.map(|v| v.into_iter());

			return self.next();

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

