// wengwengweng

use super::*;
use crate::Result;

pub trait BatchedVertex = VertexLayout + Clone;
pub trait BatchedUniform = UniformLayout + Clone + PartialEq;

#[derive(Clone, PartialEq)]
struct RenderState<V: BatchedVertex, U: BatchedUniform> {
	pipeline: Pipeline<V, U>,
	prim: Primitive,
	uniform: U,
}

// TODO: trait alias plz
pub struct BatchedMesh<V: BatchedVertex, U: BatchedUniform> {

	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	vqueue: Vec<f32>,
	iqueue: Vec<u32>,
	cur_state: Option<RenderState<V, U>>,
	draw_count: usize,

}

impl<V: BatchedVertex, U: BatchedUniform> BatchedMesh<V, U> {

	pub fn new(device: &Device, max_vertices: usize, max_indices: usize) -> Result<Self> {

		let max_vertices = max_vertices * V::STRIDE;
		let vbuf = VertexBuffer::new(&device, max_vertices, BufferUsage::Dynamic)?;
		let ibuf = IndexBuffer::new(&device, max_indices, BufferUsage::Dynamic)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::from(&device, &vbuf, Some(&ibuf))?;

		return Ok(Self {
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			vqueue: Vec::with_capacity(max_vertices),
			iqueue: Vec::with_capacity(max_indices),
			cur_state: None,
			draw_count: 0,
		});

	}

	pub fn push(
		&mut self,
		prim: Primitive,
		verts: &[f32],
		indices: &[u32],
		pipeline: &Pipeline<V, U>,
		uniform: &U,
	) -> Result<()> {

		let mut reset = false;

		if let Some(state) = &self.cur_state {
			if
				&state.pipeline != pipeline
				|| &state.uniform != uniform
				|| state.prim != prim
			{
				reset = true;
			}
		} else {
			reset = true;
		}

		if reset {
			self.flush();
			self.cur_state = Some(RenderState {
				pipeline: pipeline.clone(),
				uniform: uniform.clone(),
				prim: prim,
			});
		}

		if self.vqueue.len() + verts.len() >= self.vqueue.capacity() {
			self.flush();
		}

		if self.iqueue.len() + indices.len() >= self.iqueue.capacity() {
			self.flush();
		}

		let offset = (self.vqueue.len() / V::STRIDE) as u32;

		let indices = indices
			.iter()
			.map(|i| *i + offset)
			.collect::<Vec<u32>>();

		self.vqueue.extend_from_slice(&verts);
		self.iqueue.extend_from_slice(&indices);

		return Ok(());

	}

	pub fn push_shape<S: Shape<Vertex = V>>(
		&mut self,
		prim: Primitive,
		shape: S,
		pipeline: &Pipeline<V, U>,
		uniform: &U,
	) -> Result<()> {

		self.push(prim, &[], S::indices(), pipeline, uniform)?;
		shape.vertices(&mut self.vqueue);

		return Ok(());

	}

	pub fn flush(&mut self) {

		if self.empty() {
			return;
		}

		let state = match &self.cur_state {
			Some(s) => s,
			None => return,
		};

		self.vbuf.data(0, &self.vqueue);
		self.ibuf.data(0, &self.iqueue);

		state.pipeline.draw(
			#[cfg(feature="gl3")]
			Some(&self.vao),
			#[cfg(not(feature="gl3"))]
			Some(&self.vbuf),
			#[cfg(not(feature="gl3"))]
			Some(&self.ibuf),
			&state.uniform,
			self.iqueue.len() as u32,
			state.prim,
		);

		self.cur_state = None;
		self.vqueue.clear();
		self.iqueue.clear();
		self.draw_count += 1;

	}

	pub fn empty(&self) -> bool {
		return self.vqueue.is_empty();
	}

	pub fn clear(&mut self) {

		self.cur_state = None;
		self.vqueue.clear();
		self.iqueue.clear();
		self.draw_count = 0;

	}

	pub fn draw_count(&self) -> usize {
		return self.draw_count;
	}

	pub fn free(self) {

		self.vbuf.free();
		self.ibuf.free();
		#[cfg(feature="gl3")]
		self.vao.free();

	}

}

