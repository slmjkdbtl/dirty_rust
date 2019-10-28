// wengwengweng

use std::marker::PhantomData;

use super::*;
use crate::Result;

#[derive(Clone)]
pub struct MeshData<V: VertexLayout> {
	pub vertices: Vec<V>,
	pub indices: Vec<u32>,
}

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

	pub fn new(device: &Device, verts: &[f32], indices: &[u32]) -> Result<Self> {

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

	pub fn from_meshdata(device: &Device, data: MeshData<V>) -> Result<Self> {

		let mut verts = Vec::with_capacity(data.vertices.len() * V::STRIDE);

		for v in data.vertices {
			v.push(&mut verts);
		}

		return Self::new(device, &verts, &data.indices);

	}

	pub fn from_shape<S: Shape>(device: &Device, shape: S) -> Result<Self> {

		let mut verts = Vec::with_capacity(S::COUNT * S::Vertex::STRIDE);
		shape.vertices(&mut verts);

		return Self::new(device, &verts, &S::indices());

	}

	pub fn vbuf(&self) -> &VertexBuffer<V> {
		return &self.vbuf;
	}

	pub fn ibuf(&self) -> &IndexBuffer {
		return &self.ibuf;
	}

	pub fn draw(&self, prim: Primitive, pipeline: &Pipeline<V, U>, uniforms: Option<&U>) {

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

	pub fn drop(&self) {

		self.vbuf.drop();
		self.ibuf.drop();
		#[cfg(feature="gl3")]
		self.vao.drop();

	}

}

