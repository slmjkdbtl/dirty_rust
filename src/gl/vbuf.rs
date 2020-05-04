// wengwengweng

use std::rc::Rc;

use glow::HasContext;

use super::*;
use crate::Result;

pub trait VertexLayout: bytemuck::Pod {
	const STRIDE: usize;
	fn attrs() -> VertexAttrGroup;
}

#[derive(Clone, Debug)]
pub struct VertexBuffer<V: VertexLayout> {
	ctx: Rc<GLCtx>,
	id: BufferID,
	layout: PhantomData<V>,
}

impl<V: VertexLayout> VertexBuffer<V> {

	pub fn new(device: &Device, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_buffer()?;

			let buf = Self {
				ctx: ctx,
				id: id,
				layout: PhantomData,
			};

			buf.bind();

			buf.ctx.buffer_data_size(
				glow::ARRAY_BUFFER,
				(count * mem::size_of::<f32>()) as i32,
				usage.into(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn from(device: &Device, data: &[V]) -> Result<Self> {

		let buf = Self::new(device, data.len() * V::STRIDE, BufferUsage::Static)?;
		buf.data(0, data);
		return Ok(buf);

	}

	pub(super) fn id(&self) -> BufferID {
		return self.id;
	}

	pub(super) fn bind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.ctx.bind_buffer(glow::ARRAY_BUFFER, None);
		}
	}

	pub fn data(&self, offset: usize, data: &[V]) {

		unsafe {

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(offset * mem::size_of::<f32>()) as i32,
				bytemuck::cast_slice(data),
			);

			self.unbind();

		}

	}

	pub fn free(self) {
		unsafe {
			self.ctx.delete_buffer(self.id);
		}
	}

}

impl<V: VertexLayout> PartialEq for VertexBuffer<V> {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

