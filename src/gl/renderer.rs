// wengwengweng

use glow::Context;

use super::*;
use crate::Result;

pub struct Renderer<V: VertexLayout> {

	ctx: Rc<GLCtx>,
	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	count: usize,
	prim: Primitive,

}

impl<V: VertexLayout> Renderer<V> {

	pub fn new(device: &Device, verts: &[f32], indices: &[u32]) -> Result<Self> {

		let vbuf = VertexBuffer::<V>::init(&device, &verts)?;
		let ibuf = IndexBuffer::init(&device, &indices)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::init(&device, &vbuf)?;

		return Ok(Self {
			ctx: device.ctx.clone(),
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			count: indices.len(),
			prim: Primitive::Triangle,
		});

	}

	pub fn from_shape<S: Shape>(device: &Device, shape: S) -> Result<Self> {

		let mut verts = Vec::with_capacity(S::COUNT * S::Vertex::STRIDE);
		shape.push(&mut verts);

		return Self::new(device, &verts, &S::indices());

	}

	pub fn draw(&self, program: &Program) {
		draw(
			&self.ctx,
			#[cfg(feature="gl3")]
			&self.vao,
			#[cfg(not(feature="gl3"))]
			&self.vbuf,
			&self.ibuf,
			&program,
			self.count as u32,
			self.prim,
		);

	}

}

