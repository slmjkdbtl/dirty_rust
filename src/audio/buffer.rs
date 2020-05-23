// wengwengweng

use super::*;

#[derive(Clone)]
pub(super) struct Buffered {
	buf: Vec<f32>,
	cur_idx: usize,
}

impl Buffered {
	pub fn from_source(src: impl Source) -> Self {
		return Self {
			buf: src.into_iter().collect(),
			cur_idx: 0,
		};
	}
}

impl Iterator for Buffered {
	type Item = f32;
	fn next(&mut self) -> Option<Self::Item> {
		let v = self.buf.get(self.cur_idx).map(|f| *f);
		self.cur_idx += 1;
		return v;
	}
}

impl Source for Buffered {}

