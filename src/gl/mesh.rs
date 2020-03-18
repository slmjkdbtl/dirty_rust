// wengwengweng

use std::marker::PhantomData;

use super::*;
use crate::Result;

#[derive(Clone)]
pub struct Mesh<V: VertexLayout, U: UniformLayout> {
	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	count: usize,
	uniform_layout: PhantomData<U>,
}

impl<V: VertexLayout, U: UniformLayout> Mesh<V, U> {

	pub fn from(device: &Device, verts: &[f32], indices: &[u32]) -> Result<Self> {

		let vbuf = VertexBuffer::<V>::from(&device, &verts)?;
		let ibuf = IndexBuffer::from(&device, &indices)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::from(&device, &vbuf, Some(&ibuf))?;

		return Ok(Self {
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			count: indices.len(),
			uniform_layout: PhantomData,
		});

	}

	// TODO: name
	pub fn from2(device: &Device, verts: &[V], indices: &[u32]) -> Result<Self> {

		let mut queue = Vec::with_capacity(verts.len() * V::STRIDE);

		for v in verts {
			v.push(&mut queue);
		}

		return Self::from(device, &queue, &indices);

	}

	pub fn from_shape<S: Shape<Vertex = V>>(device: &Device, shape: S) -> Result<Self> {

		let mut verts = Vec::with_capacity(S::COUNT * S::Vertex::STRIDE);
		shape.vertices(&mut verts);

		return Self::from(device, &verts, &S::indices());

	}

	pub fn vbuf(&self) -> &VertexBuffer<V> {
		return &self.vbuf;
	}

	pub fn ibuf(&self) -> &IndexBuffer {
		return &self.ibuf;
	}

	pub fn draw(&self, prim: Primitive, pipeline: &Pipeline<V, U>, uniforms: &U) {

		pipeline.draw(
			#[cfg(feature="gl3")]
			Some(&self.vao),
			#[cfg(not(feature="gl3"))]
			Some(&self.vbuf),
			#[cfg(not(feature="gl3"))]
			Some(&self.ibuf),
			uniforms,
			self.count as u32,
			prim,
		);

	}

	pub fn free(self) {

		self.vbuf.free();
		self.ibuf.free();
		#[cfg(feature="gl3")]
		self.vao.free();

	}

}

