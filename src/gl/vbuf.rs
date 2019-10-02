// wengwengweng

use std::rc::Rc;

use glow::Context;

use super::*;
use crate::Result;

pub trait VertexLayout {

	const STRIDE: usize;
	fn push(&self, queue: &mut Vec<f32>);
	fn attrs() -> VertexAttrGroup;

}

#[derive(Clone, Debug)]
pub struct VertexBuffer<V: VertexLayout> {

	ctx: Rc<GLCtx>,
	pub(super) id: BufferID,
	pub(super) attrs: VertexAttrGroup,
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
				attrs: V::attrs(),
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

	pub fn init(device: &Device, data: &[f32]) -> Result<Self> {

		let buf = Self::new(device, data.len(), BufferUsage::Static)?;
		buf.data(0, data);
		return Ok(buf);

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

	// TODO: put this elsewhere?
	pub(super) fn bind_attrs<U: UniformInterface>(&self, program: &Program<U>) {

		unsafe {

			for attr in iter_attrs(&self.attrs) {

				let index = self.ctx.get_attrib_location(program.id, &attr.name) as u32;

				self.ctx.vertex_attrib_pointer_f32(
					index,
					attr.size,
					glow::FLOAT,
					false,
					(V::STRIDE * mem::size_of::<f32>()) as i32,
					(attr.offset * mem::size_of::<f32>()) as i32,
				);

				self.ctx.enable_vertex_attrib_array(index);

			}

		}

	}

	// TODO: change this to take V?
	pub fn data(&self, offset: usize, data: &[f32]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(offset * mem::size_of::<f32>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

	pub fn data_raw<T>(&self, data: &[T]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.ctx.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				0,
				byte_slice,
			);

			self.unbind();

		}

	}

}

impl<V: VertexLayout> Drop for VertexBuffer<V> {
	fn drop(&mut self) {
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

