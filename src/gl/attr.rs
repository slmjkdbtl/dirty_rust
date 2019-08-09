// wengwengweng

#[derive(Clone, Debug)]
pub struct VertexAttrGroup {
	attrs: Vec<VertexAttr>,
	cur_offset: usize,
}

impl VertexAttrGroup {

	pub fn build() -> Self {
		return Self {
			attrs: Vec::new(),
			cur_offset: 0,
		};
	}

	pub fn iter(&self) -> std::slice::Iter<VertexAttr> {
		return self.attrs.iter();
	}

	pub fn add(mut self, name: &str, size: u8) -> Self {

		self.attrs.push(VertexAttr {
			name: name.to_owned(),
			size: size as i32,
			offset: self.cur_offset,
		});

		self.cur_offset += size as usize;

		return self;

	}

}

impl<'a> IntoIterator for &'a VertexAttrGroup {

	type Item = &'a VertexAttr;
	type IntoIter = std::slice::Iter<'a, VertexAttr>;

	fn into_iter(self) -> Self::IntoIter {
		return self.attrs.iter();
	}

}

#[derive(Clone, Debug)]
pub struct VertexAttr {

	pub name: String,
	pub size: i32,
	pub offset: usize,

}

