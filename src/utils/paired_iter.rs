// wengwengweng

pub struct PairedIter<'a, T> {
	list: &'a [T],
	index: usize,
}

impl<'a, T> Iterator for PairedIter<'a, T> {

	type Item = (&'a T, &'a T);

	fn next(&mut self) -> Option<(&'a T, &'a T)> {

		if let Some(i1) = self.list.get(self.index) {
			let i2 = self.list.get(self.index + 1).unwrap_or(self.list.get(0).unwrap());
			self.index += 1;
			return Some((i1, i2));
		} else {
			return None;
		}

	}

}

pub fn paired<T>(v: &[T]) -> PairedIter<T> {
	return PairedIter {
		list: v,
		index: 0,
	};
}

