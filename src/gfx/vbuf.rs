// wengwengweng

use std::mem;
use std::rc::Rc;
use std::marker::PhantomData;

use glow::HasContext;

use super::*;
use crate::Result;

pub type VertexAttrGroup = &'static[(&'static str, u8)];

pub(super) struct VertexAttrIter {
	attrs: VertexAttrGroup,
	cur_offset: usize,
	cur_idx: usize,
}

pub(super) fn iter_attrs(attrs: VertexAttrGroup) -> VertexAttrIter {
	return VertexAttrIter {
		attrs: attrs,
		cur_offset: 0,
		cur_idx: 0,
	};
}

impl Iterator for VertexAttrIter {

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
pub(super) struct VertexAttr {
	pub name: &'static str,
	pub size: i32,
	pub offset: usize,
}

pub(super) trait VertexLayout: Clone {
	fn attrs() -> VertexAttrGroup;
}

#[derive(Clone, Debug)]
pub(super) struct VertexBuffer<V: VertexLayout> {
	gl: Rc<glow::Context>,
	id: BufferID,
	_layout: PhantomData<V>,
}

impl<V: VertexLayout> VertexBuffer<V> {

	pub fn new(ctx: &impl GLCtx, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let id = gl.create_buffer()?;

			let buf = Self {
				gl,
				id,
				_layout: PhantomData,
			};

			buf.bind();

			buf.gl.buffer_data_size(
				glow::ARRAY_BUFFER,
				(count * mem::size_of::<V>()) as i32,
				usage.to_glow(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn from(ctx: &impl GLCtx, data: &[V]) -> Result<Self> {

		let buf = Self::new(ctx, data.len(), BufferUsage::Static)?;
		buf.data(0, data);
		return Ok(buf);

	}

	pub(super) fn id(&self) -> BufferID {
		return self.id;
	}

	pub(super) fn bind(&self) {
		unsafe {
			self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
		}
	}

	pub fn data(&self, offset: usize, data: &[V]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.gl.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(offset * mem::size_of::<V>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

	pub fn free(self) {
		unsafe {
			self.gl.delete_buffer(self.id);
		}
	}

}

impl<V: VertexLayout> PartialEq for VertexBuffer<V> {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

