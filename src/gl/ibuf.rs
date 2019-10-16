// wengwengweng

use std::rc::Rc;

use glow::HasContext;

use super::*;
use crate::Result;

#[derive(Clone, Debug)]
pub struct IndexBuffer {

	ctx: Rc<GLCtx>,
	id: BufferID,

}

impl IndexBuffer {

	pub fn new(device: &Device, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_buffer()?;

			let buf = Self {
				ctx: ctx,
				id: id,
			};

			buf.bind();

			buf.ctx.buffer_data_size(
				glow::ELEMENT_ARRAY_BUFFER,
				(count * mem::size_of::<u32>()) as i32,
				usage.into(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn from(device: &Device, data: &[u32]) -> Result<Self> {

		let buf = Self::new(device, data.len(), BufferUsage::Static)?;
		buf.data(0, data);
		return Ok(buf);

	}

	pub(super) fn id(&self) -> BufferID {
		return self.id;
	}

	pub(super) fn bind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
		}
	}

	pub fn data(&self, offset: usize, data: &[u32]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ELEMENT_ARRAY_BUFFER,
				(offset * mem::size_of::<u32>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

	pub fn drop(&self) {
		unsafe {
			self.ctx.delete_buffer(self.id);
		}
	}

}

impl PartialEq for IndexBuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}


