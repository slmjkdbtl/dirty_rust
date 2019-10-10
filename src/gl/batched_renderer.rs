// wengwengweng

use super::*;
use crate::Result;

// TODO: trait alias plz
// TODO: shouldn't need to bind to one type of UniformLayout
pub struct BatchedRenderer<V: VertexLayout + Clone, U: UniformLayout + PartialEq + Clone> {

	ctx: Rc<GLCtx>,
	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	vqueue: Vec<f32>,
	iqueue: Vec<u32>,
	prim: Primitive,
	cur_uniform: Option<U>,
	cur_pipeline: Option<Pipeline<V, U>>,
	draw_count: usize,

}

impl<V: VertexLayout + Clone, U: UniformLayout + PartialEq + Clone> BatchedRenderer<V, U> {

	pub fn new(device: &Device, max_vertices: usize, max_indices: usize) -> Result<Self> {

		let vbuf = VertexBuffer::new(&device, max_vertices, BufferUsage::Dynamic)?;
		let ibuf = IndexBuffer::new(&device, max_indices, BufferUsage::Dynamic)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::from(&device, &vbuf, Some(&ibuf))?;

		return Ok(Self {
			ctx: device.ctx.clone(),
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			vqueue: Vec::with_capacity(max_vertices),
			iqueue: Vec::with_capacity(max_indices),
			prim: Primitive::Triangle,
			cur_uniform: None,
			cur_pipeline: None,
			draw_count: 0,
		});

	}

	pub fn push(
		&mut self,
		verts: &[f32],
		indices: &[u32],
		pipeline: &Pipeline<V, U>,
		uniform: &U,
	) -> Result<()> {

		// TODO: don't use recursion
		if let Some(cur_pipeline) = &self.cur_pipeline {
			if cur_pipeline != pipeline {
				self.flush();
				return self.push(verts, indices, pipeline, uniform);
			}
		} else {
			self.cur_pipeline = Some(pipeline.clone());
		}

		if let Some(cur_uniform) = &self.cur_uniform {
			if cur_uniform != uniform {
				self.flush();
				return self.push(verts, indices, pipeline, uniform);
			}
		} else {
			self.cur_uniform = Some(uniform.clone());
		}

		if self.vqueue.len() + verts.len() >= self.vqueue.capacity() {
			self.flush();
			return self.push(verts, indices, pipeline, uniform);
		}

		if self.iqueue.len() + indices.len() >= self.iqueue.capacity() {
			self.flush();
			return self.push(verts, indices, pipeline, uniform);
		}

		let offset = (self.vqueue.len() / V::STRIDE) as u32;

		let indices = indices
			.iter()
			.map(|i| *i + offset)
			.collect::<Vec<u32>>();
			;

		self.vqueue.extend_from_slice(&verts);
		self.iqueue.extend_from_slice(&indices);

		return Ok(());

	}

	pub fn push_shape<S: Shape>(
		&mut self,
		shape: S,
		pipeline: &Pipeline<V, U>,
		uniform: &U,
	) -> Result<()> {

		self.push(&[], S::indices(), pipeline, uniform)?;
		shape.vertices(&mut self.vqueue);

		return Ok(());

	}

	pub fn flush(&mut self) {

		if self.empty() {
			return;
		}

		let pipeline = match &self.cur_pipeline {
			Some(p) => p,
			None => return,
		};

		let uniform = match &self.cur_uniform {
			Some(p) => p,
			None => return,
		};

		self.vbuf.data(0, &self.vqueue);
		self.ibuf.data(0, &self.iqueue);

		pipeline.draw(
			#[cfg(feature="gl3")]
			Some(&self.vao),
			#[cfg(not(feature="gl3"))]
			Some(&self.vbuf),
			#[cfg(not(feature="gl3"))]
			Some(&self.ibuf),
			Some(uniform),
			self.iqueue.len() as u32,
			self.prim,
		);

		self.cur_pipeline = None;
		self.cur_uniform = None;
		self.vqueue.clear();
		self.iqueue.clear();
		self.draw_count += 1;

	}

	pub fn empty(&self) -> bool {
		return self.vqueue.is_empty();
	}

	pub fn clear(&mut self) {

		self.cur_pipeline = None;
		self.cur_uniform = None;
		self.vqueue.clear();
		self.iqueue.clear();
		self.draw_count = 0;

	}

	pub fn draw_count(&self) -> usize {
		return self.draw_count;
	}

}

