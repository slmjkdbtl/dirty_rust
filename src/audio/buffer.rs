// wengwengweng

use std::sync::Arc;

use super::*;

#[derive(Clone)]
pub(super) struct Buffered {
	buf: Arc<Vec<Frame>>,
	cur_idx: usize,
}

impl Buffered {
	pub fn new(src: impl Source) -> Self {
		return Self {
			buf: Arc::new(src.into_iter().collect()),
			cur_idx: 0,
		};
	}
}

impl Iterator for Buffered {
	type Item = Frame;
	fn next(&mut self) -> Option<Self::Item> {
		let v = self.buf.get(self.cur_idx).map(|f| *f);
		self.cur_idx += 1;
		return v;
	}
}

impl Source for Buffered {}

