// wengwengweng

use std::marker::PhantomData;

use super::*;
use crate::Result;

pub struct Renderer<V: VertexLayout, U: UniformLayout> {

	ctx: Rc<GLCtx>,
	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	count: usize,
	prim: Primitive,
	uniform_layout: PhantomData<U>,

}

impl<V: VertexLayout, U: UniformLayout> Renderer<V, U> {

	pub fn new(device: &Device, verts: &[f32], indices: &[u32]) -> Result<Self> {

		let vbuf = VertexBuffer::<V>::from(&device, &verts)?;
		let ibuf = IndexBuffer::from(&device, &indices)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::from(&device, &vbuf, Some(&ibuf))?;

		return Ok(Self {
			ctx: device.ctx.clone(),
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			count: indices.len(),
			prim: Primitive::Triangle,
			uniform_layout: PhantomData,
		});

	}

	pub fn from_shape<S: Shape>(device: &Device, shape: S) -> Result<Self> {

		let mut verts = Vec::with_capacity(S::COUNT * S::Vertex::STRIDE);
		shape.vertices(&mut verts);

		return Self::new(device, &verts, &S::indices());

	}

	pub fn draw(&self, pipeline: &Pipeline<V, U>, uniforms: Option<&U>) {

		pipeline.draw(
			#[cfg(feature="gl3")]
			Some(&self.vao),
			#[cfg(not(feature="gl3"))]
			Some(&self.vbuf),
			#[cfg(not(feature="gl3"))]
			Some(&self.ibuf),
			uniforms,
			self.count as u32,
			self.prim,
		);

	}

}

