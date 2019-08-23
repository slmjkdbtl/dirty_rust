// wengwengweng

use std::rc::Rc;

use glow::Context;

use super::*;
use crate::Result;

type VertexArrayID = <GLCtx as Context>::VertexArray;

#[derive(Clone, Debug)]
pub struct VertexArray {
	ctx: Rc<GLCtx>,
	pub(super) id: VertexArrayID,
}

impl VertexArray {

	pub fn new(device: &Device) -> Result<Self> {

		unsafe {

			let ctx = device.ctx.clone();
			let id = ctx.create_vertex_array()?;

			let buf = Self {
				ctx: ctx,
				id: id,
			};

			return Ok(buf);

		}

	}

	pub fn init<V: VertexLayout>(device: &Device, vbuf: &VertexBuffer<V>) -> Result<Self> {

		let vao = Self::new(device)?;
		vao.attr(vbuf);
		return Ok(vao);

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.ctx.bind_vertex_array(Some(self.id));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.ctx.bind_vertex_array(None);
		}
	}

	pub fn attr<V: VertexLayout>(&self, vbuf: &VertexBuffer<V>) {

		unsafe {

			self.bind();
			vbuf.bind();

			for (i, attr) in V::attrs().iter().enumerate() {

				self.ctx.vertex_attrib_pointer_f32(
					i as u32,
					attr.size,
					glow::FLOAT,
					false,
					(V::STRIDE * mem::size_of::<f32>()) as i32,
					(attr.offset * mem::size_of::<f32>()) as i32,
				);

				self.ctx.enable_vertex_attrib_array(i as u32);

			}

			vbuf.unbind();
			self.unbind();

		}

	}

}

impl Drop for VertexArray {
	fn drop(&mut self) {
		unsafe {
			self.ctx.delete_vertex_array(self.id);
		}
	}
}

impl PartialEq for VertexArray {
	fn eq(&self, other: &Self) -> bool {
		return self.id == other.id;
	}
}

