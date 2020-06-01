// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;

use super::*;

pub struct Converter {
	src: Arc<Mutex<dyn Source + Send>>,
	target: Spec,
	prev_frame: Option<Frame>,
	next_frame: Option<Frame>,
	pos: f32,
	frame_pos: usize,
}

impl Converter {
	pub fn new(src: Arc<Mutex<dyn Source + Send>>, target: Spec) -> Result<Self> {
		return Ok(Self {
			src: src,
			target: target,
			prev_frame: None,
			next_frame: None,
			pos: 0.0,
			frame_pos: 0,
		})
	}
	pub fn get_inner(&self) -> &Arc<Mutex<dyn Source + Send>> {
		return &self.src;
	}
}

impl Iterator for Converter {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		let mut src = match self.src.lock() {
			Ok(src) => src,
			Err(_) => return None,
		};

		let sample_rate = src.sample_rate();

		if self.target.sample_rate == sample_rate {
			return src.next();
		}

		// TODO: bugged yo
		let speed = sample_rate as f32 / self.target.sample_rate as f32;

		self.pos += speed;

		if self.pos > self.frame_pos as f32 {
			let skip = self.pos as usize - self.frame_pos;
			for _ in 0..skip {
				src.next();
			}
			self.prev_frame = self.next_frame;
			self.next_frame = src.next();

			if self.next_frame.is_none() {
				return None;
			}

			self.frame_pos += skip + 1;
			let progress = self.pos - (self.frame_pos - 1) as f32;
			let prev = self.prev_frame.unwrap_or_default();
			let next = self.next_frame.unwrap_or_default();
			return Some(prev + (next - prev) * progress);
		} else {
			let prev = self.prev_frame.unwrap_or_default();
			let next = self.next_frame.unwrap_or_default();
			let progress = self.pos - (self.frame_pos - 1) as f32;
			return Some(prev + (next - prev) * progress);
		}

	}

}

