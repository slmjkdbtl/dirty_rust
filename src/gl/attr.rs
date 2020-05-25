// wengwengweng

pub type VertexAttrGroup = &'static[(&'static str, u8)];

pub(super) struct VertexAttrIter<'a> {
	attrs: &'a VertexAttrGroup,
	cur_offset: usize,
	cur_idx: usize,
}

pub(super) fn iter_attrs<'a>(attrs: &'a VertexAttrGroup) -> VertexAttrIter<'a> {
	return VertexAttrIter {
		attrs,
		cur_offset: 0,
		cur_idx: 0,
	};
}

impl<'a> Iterator for VertexAttrIter<'a> {

	type Item = VertexAttr;

	fn next(&mut self) -> Option<Self::Item> {

		if let Some(data) = self.attrs.get(self.cur_idx) {

			let attr = VertexAttr {
				name: data.0,
				size: data.1 as i32,
				offset: self.cur_offset,
			};

			self.cur_offset += data.1 as usize;
			self.cur_idx += 1;

			return Some(attr);

		} else {

			return None;

		}

	}

}

#[derive(Clone, Debug)]
pub struct VertexAttr {
	pub name: &'static str,
	pub size: i32,
	pub offset: usize,
}

