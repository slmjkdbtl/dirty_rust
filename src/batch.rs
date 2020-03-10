// wengwengweng

use std::marker::PhantomData;
use crate::*;

pub struct BatchedMesh<V: gl::VertexLayout, U: gl::UniformLayout> {
	vbuf: gl::VertexBuffer<V>,
	ibuf: gl::IndexBuffer,
	uniform_layout: PhantomData<U>,
}

impl<V: gl::VertexLayout, U: gl::UniformLayout> BatchedMesh<V, U> {

	pub fn new(ctx: &impl gfx::GfxCtx, max_vertices: usize, max_indices: usize) -> Result<Self> {

		let max_vertices = max_vertices * V::STRIDE;
		let vbuf = gl::VertexBuffer::new(ctx.device(), max_vertices, gl::BufferUsage::Dynamic)?;
		let ibuf = gl::IndexBuffer::new(ctx.device(), max_indices, gl::BufferUsage::Dynamic)?;

		return Ok(Self {
			vbuf: vbuf,
			ibuf: ibuf,
			uniform_layout: PhantomData,
		});

	}

	pub fn draw(
		&self,
		vertices: &[f32],
		indices: &[u32],
		state: gl::RenderState<U>
	) -> Result<()> {

		self.vbuf.data(0, vertices);
		self.ibuf.data(0, indices);

		return Ok(());

	}

}

