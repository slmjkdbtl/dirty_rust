// wengwengweng

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use super::*;

pub struct Mp3Decoder<R: Read + Seek> {
	decoder: puremp3::Mp3Decoder<R>,
	cur_frame: puremp3::Frame,
	cur_frame_offset: usize,
}

impl<R: Read + Seek> Mp3Decoder<R> {

	pub fn new(data: R) -> Result<Self> {

		let mut decoder = puremp3::Mp3Decoder::new(data);
		let cur_frame = decoder.next_frame().map_err(|_| format!("failed to parse mp3"))?;

		return Ok(Self {
			decoder: decoder,
			cur_frame: cur_frame,
			cur_frame_offset: 0,
		});

	}

}

impl<R: Read + Seek> Source for Mp3Decoder<R> {}

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

		return Some((left, right));

	}

}

pub fn is_mp3<R: Read + Seek>(mut data: R) -> bool {

	let pos = match data.seek(SeekFrom::Current(0)) {
		Ok(pos) => pos,
		Err(_) => return false,
	};

	let mut decoder = puremp3::Mp3Decoder::new(data.by_ref());

	if decoder.next_frame().is_err() {
		data.seek(SeekFrom::Start(pos)).expect("cannot seek to start");
		return false;
	}

	data.seek(SeekFrom::Start(pos)).expect("cannot seek to start");

	return true;

}

