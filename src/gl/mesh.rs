// wengwengweng

use std::marker::PhantomData;

use super::*;
use crate::Result;

pub struct Mesh<V: VertexLayout, U: UniformLayout> {

	vbuf: VertexBuffer<V>,
	ibuf: Option<IndexBuffer>,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	count: usize,
	prim: Primitive,
	uniform_layout: PhantomData<U>,

}

impl<V: VertexLayout, U: UniformLayout> Mesh<V, U> {

	pub fn new_no_index(device: &Device, verts: &[f32]) -> Result<Self> {

		let vbuf = VertexBuffer::<V>::from(&device, &verts)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::from(&device, &vbuf, None)?;

		return Ok(Self {
			vbuf: vbuf,
			ibuf: None,
			#[cfg(feature="gl3")]
			vao: vao,
			count: verts.len(),
			prim: Primitive::Triangle,
			uniform_layout: PhantomData,
		});

	}

	pub fn new(device: &Device, verts: &[f32], indices: &[u32]) -> Result<Self> {

		let mut mesh = Self::new_no_index(device, verts)?;
		let ibuf = IndexBuffer::from(&device, &indices)?;

		#[cfg(feature="gl3")]
		mesh.vao.bind_ibuf(&ibuf);

		mesh.count = indices.len();
		mesh.ibuf = Some(ibuf);

		return Ok(mesh);

	}

	pub fn from_shape<S: Shape>(device: &Device, shape: S) -> Result<Self> {

		let mut verts = Vec::with_capacity(S::COUNT * S::Vertex::STRIDE);
		shape.vertices(&mut verts);

		if let Some(indices) = S::indices() {
			return Self::new(device, &verts, indices);
		} else {
			return Self::new_no_index(device, &verts);
		}

	}

	pub fn draw(&self, pipeline: &Pipeline<V, U>, uniforms: Option<&U>) {

		pipeline.draw(
			#[cfg(feature="gl3")]
			Some(&self.vao),
			#[cfg(not(feature="gl3"))]
			Some(&self.vbuf),
			#[cfg(not(feature="gl3"))]
			self.ibuf.as_ref(),
			uniforms,
			self.count as u32,
			self.prim,
		);

	}

	pub fn drop(&self) {

		self.vbuf.drop();
		self.ibuf.as_ref().map(|i| i.drop());
		#[cfg(feature="gl3")]
		self.vao.drop();

	}

}

