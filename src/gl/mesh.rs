// wengwengweng

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

	pub fn from(device: &Device, verts: &[V], indices: &[u32]) -> Result<Self> {

		let vbuf = VertexBuffer::<V>::from(&device, &verts)?;
		let ibuf = IndexBuffer::from(&device, &indices)?;

		return Ok(Self {
			vbuf: vbuf,
			ibuf: ibuf,
			count: indices.len(),
			_uniform_layout: PhantomData,
		});

	}

	pub fn from_shape<S: Shape<Vertex = V>>(device: &Device, shape: S) -> Result<Self> {
		return Self::from(device, &shape.vertices(), &S::indices());
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

