// wengwengweng

use std::marker::PhantomData;

use super::*;
use crate::Result;

#[derive(Clone)]
pub struct Mesh<V: VertexLayout, U: UniformLayout> {
	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	count: usize,
	_uniform_layout: PhantomData<U>,
}

impl<V: VertexLayout, U: UniformLayout> Mesh<V, U> {

	pub fn new(ctx: &impl HasGL, verts: &[V], indices: &[u32]) -> Result<Self> {

		let vbuf = VertexBuffer::<V>::from(ctx, &verts)?;
		let ibuf = IndexBuffer::from(ctx, &indices)?;

		return Ok(Self {
			vbuf,
			ibuf,
			count: indices.len(),
			_uniform_layout: PhantomData,
		});

	}

	pub fn vbuf(&self) -> &VertexBuffer<V> {
		return &self.vbuf;
	}

	pub fn ibuf(&self) -> &IndexBuffer {
		return &self.ibuf;
	}

	pub fn draw(&self, prim: Primitive, pipeline: &Pipeline<V, U>, uniforms: &U) {
		pipeline.draw(
			Some(&self.vbuf),
			Some(&self.ibuf),
			uniforms,
			self.count as u32,
			prim,
		);
	}

	pub fn free(self) {
		self.vbuf.free();
		self.ibuf.free();
	}

}

