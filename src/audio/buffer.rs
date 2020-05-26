// wengwengweng

use std::time::Duration;
use std::sync::Arc;

use super::*;

#[derive(Clone)]
pub struct AudioBuffer {
	frames: Vec<Frame>,
	sample_rate: u32,
	duration: Duration,
}

impl AudioBuffer {

	pub fn from_source(src: impl Source) -> Self {

		let sample_rate = src.sample_rate();
		let frames = src.into_iter().collect::<Vec<Frame>>();

		return Self {
			sample_rate: sample_rate,
			duration: Duration::from_secs_f32(frames.len() as f32 / sample_rate as f32),
			frames: frames,
		};

	}

	pub fn duration(&self) -> Duration {
		return self.duration;
	}

}

// TODO: bad name
#[derive(Clone)]
pub(super) struct Buffered {
	buf: Arc<Vec<Frame>>,
	sample_rate: u32,
	cur_pos: usize,
	duration: Duration,
}

impl Buffered {

	pub fn new(src: impl Source) -> Self {

		let sample_rate = src.sample_rate();
		let buf = src.into_iter().collect::<Vec<Frame>>();

		return Self {
			sample_rate: sample_rate,
			duration: Duration::from_secs_f32(buf.len() as f32 / sample_rate as f32),
			buf: Arc::new(buf),
			cur_pos: 0,
		};

	}

	pub fn duration(&self) -> Duration {
		return self.duration;
	}

}

impl Iterator for Buffered {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {
		let v = self.buf.get(self.cur_pos).map(|f| *f);
		self.cur_pos += 1;
		return v;
	}

}

impl Source for Buffered {

	fn sample_rate(&self) -> u32 {
		return self.sample_rate;
	}

	fn seek_start(&mut self) -> Result<()> {
		self.cur_pos = 0;
		return Ok(());
	}

}

