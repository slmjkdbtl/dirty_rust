// wengwengweng

use std::time::Duration;
use std::sync::Arc;

use super::*;

#[derive(Clone)]
pub(super) struct Buffered {
	buf: Arc<Vec<Frame>>,
	sample_rate: u32,
	cur_idx: usize,
}

impl Buffered {
	pub fn new(src: impl Source) -> Self {
		return Self {
			sample_rate: src.sample_rate(),
			buf: Arc::new(src.collect()),
			cur_idx: 0,
		};
	}
	pub fn duration(&self) -> Duration {
		return Duration::from_secs_f32(self.buf.len() as f32 / self.sample_rate as f32);
	}
}

impl Iterator for Buffered {
	type Item = Frame;
	fn next(&mut self) -> Option<Self::Item> {
		let v = self.buf.get(self.cur_idx).copied();
		self.cur_idx += 1;
		return v;
	}
}

impl Source for Buffered {
	fn sample_rate(&self) -> u32 {
		return self.sample_rate;
	}
}

