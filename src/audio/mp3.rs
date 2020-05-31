// wengwengweng

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use super::*;

pub struct Mp3Decoder<R: Read + Seek> {
	decoder: puremp3::Mp3Decoder<R>,
	cur_frame: puremp3::Frame,
	cur_frame_offset: usize,
	sample_rate: u32,
}

impl<R: Read + Seek> Mp3Decoder<R> {

	pub fn new(data: R) -> Result<Self> {

		let mut decoder = puremp3::Mp3Decoder::new(data);
		let cur_frame = decoder
			.next_frame()
			.map_err(|_| format!("failed to parse mp3"))?;
		let header = &cur_frame.header;
		let sample_rate = header.sample_rate.hz();

		return Ok(Self {
			decoder,
			cur_frame,
			cur_frame_offset: 0,
			sample_rate,
		});

	}

}

impl<R: Read + Seek> Source for Mp3Decoder<R> {

	fn sample_rate(&self) -> u32 {
		return self.sample_rate;
	}

	fn seek_start(&mut self) -> Result<()> {

		let reader = self.decoder.get_mut();

		reader
			.seek(SeekFrom::Start(0))
			.map_err(|_| format!("failed to seek mp3"))?;

		self.cur_frame_offset = 0;

		return Ok(());

	}

}

impl<R: Read + Seek> Iterator for Mp3Decoder<R> {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		if self.cur_frame_offset == self.cur_frame.samples[0].len() {
			self.cur_frame_offset = 0;
			match self.decoder.next_frame() {
				Ok(frame) => self.cur_frame = frame,
				_ => return None,
			}
		}

		let left = self.cur_frame.samples[0][self.cur_frame_offset];
		let right = self.cur_frame.samples[1][self.cur_frame_offset];

		self.cur_frame_offset += 1;

		return Some(Frame::new(left, right));

	}

}

pub fn is_mp3<R: Read + Seek>(mut reader: R) -> Result<bool> {

	let pos = reader
		.seek(SeekFrom::Current(0))
		.map_err(|_| format!("failed to seek"))?;

	let mut decoder = puremp3::Mp3Decoder::new(&mut reader);
	let is_mp3 = decoder.next_frame().is_ok();

	reader
		.seek(SeekFrom::Start(pos))
		.map_err(|_| format!("failed to seek"))?;

	return Ok(is_mp3);

}

