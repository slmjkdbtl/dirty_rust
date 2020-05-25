// wengwengweng

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::vec;

use lewton::inside_ogg::OggStreamReader;
use lewton::samples::InterleavedSamples;

use super::*;

pub struct VorbisDecoder<R: Read + Seek> {
	// TODO: have to do this for seek_start cuz seek_absgp_pg won't work
	decoder: Option<OggStreamReader<R>>,
	cur_packet: Option<vec::IntoIter<f32>>,
	channel_count: ChannelCount,
	sample_rate: u32,
}

impl<R: Read + Seek> VorbisDecoder<R> {

	pub fn new(reader: R) -> Result<Self> {

		let mut decoder = OggStreamReader::new(reader)
			.map_err(|_| "failed to parse vorbis".to_string())?;

		let header = &decoder.ident_hdr;

		let channel_count = match header.audio_channels {
			1 => ChannelCount::One,
			2 => ChannelCount::Two,
			_ => return Err(format!("unsupported channel count: {}", header.audio_channels)),
		};

		let sample_rate = header.audio_sample_rate;

		let data = match decoder.read_dec_packet_generic::<InterleavedSamples<f32>>() {
			Ok(data) => data,
			Err(e) => return Err("failed to read vorbis".to_string()),
		};

		return Ok(Self {
			decoder: Some(decoder),
			cur_packet: data.map(|d| d.samples.into_iter()),
			channel_count,
			sample_rate,
		});

	}

	fn next_sample(&mut self) -> Option<f32> {

		let decoder = match &mut self.decoder {
			Some(decoder) => decoder,
			None => return None,
		};

		let cur_packet = match &mut self.cur_packet {
			Some(packet) => packet,
			None => return None,
		};

		if let Some(sample) = cur_packet.next() {
			return Some(sample);
		} else {
			self.cur_packet = decoder
				.read_dec_packet_generic::<InterleavedSamples<f32>>()
				.ok()
				.flatten()
				.map(|v| v.samples.into_iter());
			return self.next_sample();
		}

	}

}

impl<R: Read + Seek> Source for VorbisDecoder<R> {

	fn sample_rate(&self) -> u32 {
		return self.sample_rate;
	}

	fn seek_start(&mut self) -> Result<()> {

		let decoder = match self.decoder.take() {
			Some(decoder) => decoder,
			None => return Err("failed to seek vorbis".to_string()),
		};

		let mut reader = decoder.into_inner().into_inner();

		reader
			.seek(SeekFrom::Start(0))
			.map_err(|_| "failed to seek mp3".to_string())?;

		let mut decoder = OggStreamReader::new(reader)
			.map_err(|_| "failed to parse vorbis".to_string())?;

		let data = match decoder.read_dec_packet_generic::<InterleavedSamples<f32>>() {
			Ok(data) => data,
			Err(e) => return Err("failed to read vorbis".to_string()),
		};

		self.decoder = Some(decoder);
		self.cur_packet = data.map(|d| d.samples.into_iter());

		return Ok(());

	}

}

impl<R: Read + Seek> Iterator for VorbisDecoder<R> {

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

pub fn is_vorbis<R: Read + Seek>(mut reader: R) -> Result<bool> {

	let pos = reader
		.seek(SeekFrom::Current(0))
		.map_err(|_| "failed to seek".to_string())?;

	let is_vorbis = OggStreamReader::new(&mut reader).is_ok();

	reader
		.seek(SeekFrom::Start(pos))
		.map_err(|_| "failed to seek".to_string())?;

	return Ok(is_vorbis)

}

