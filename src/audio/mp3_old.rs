// wengwengweng

use std::time::Duration;
use std::io::Read;
use std::io::Seek;

pub struct Mp3Decoder<R: Read + Seek> {
	decoder: minimp3::Decoder<R>,
	cur_frame: minimp3::Frame,
	cur_frame_offset: usize,
}

impl<R: Read + Seek> Mp3Decoder<R> {

	pub fn from_reader(data: R) -> Result<Self, String> {

		let mut mp3 = minimp3::Decoder::new(data);
		let frame = mp3.next_frame().map_err(|_| format!("failed to get mp3 frames"))?;

		return Ok(Self {
			decoder: mp3,
			cur_frame: frame,
			cur_frame_offset: 0,
		});

	}

}

impl<R: Read + Seek> Iterator for Mp3Decoder<R> {

	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {

		if self.cur_frame_offset == self.cur_frame.data.len() {
			self.cur_frame_offset = 0;
			match self.decoder.next_frame() {
				Ok(frame) => self.cur_frame = frame,
				_ => return None,
			}
		}

		let v = self.cur_frame.data[self.cur_frame_offset];
		self.cur_frame_offset += 1;

		return Some(v as f32 / i16::max_value() as f32);

	}

}

