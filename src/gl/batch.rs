// wengwengweng

use super::*;
use crate::Result;

// TODO: trait alias plz
pub struct BatchedMesh<V: VertexLayout + Clone, U: UniformLayout + PartialEq + Clone> {

	vbuf: VertexBuffer<V>,
	ibuf: Option<IndexBuffer>,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	vqueue: Vec<f32>,
	iqueue: Option<Vec<u32>>,
	prim: Primitive,
	cur_uniform: Option<U>,
	cur_pipeline: Option<Pipeline<V, U>>,
	draw_count: usize,

}

impl<V: VertexLayout + Clone, U: UniformLayout + PartialEq + Clone> BatchedMesh<V, U> {

	pub fn new_no_index(device: &Device, max_vertices: usize) -> Result<Self> {

		let max_vertices = max_vertices * V::STRIDE;
		let vbuf = VertexBuffer::new(&device, max_vertices, BufferUsage::Dynamic)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::from(&device, &vbuf, None)?;

		return Ok(Self {
			vbuf: vbuf,
			ibuf: None,
			#[cfg(feature="gl3")]
			vao: vao,
			vqueue: Vec::with_capacity(max_vertices),
			iqueue: None,
			prim: Primitive::Triangle,
			cur_uniform: None,
			cur_pipeline: None,
			draw_count: 0,
		});

	}

	pub fn new(device: &Device, max_vertices: usize, max_indices: usize) -> Result<Self> {

		let mut mesh = Self::new_no_index(device, max_vertices)?;
		let ibuf = IndexBuffer::new(&device, max_indices, BufferUsage::Dynamic)?;

		#[cfg(feature="gl3")]
		mesh.vao.bind_ibuf(ibuf);

		mesh.ibuf = Some(ibuf);
		mesh.iqueue = Some(Vec::with_capacity(max_indices));

		return Ok(mesh);

	}

	pub fn push_no_index(
		&mut self,
		verts: &[f32],
		pipeline: &Pipeline<V, U>,
		uniform: &U,
	) -> Result<()> {

		// TODO: don't use recursion
		if let Some(cur_pipeline) = &self.cur_pipeline {
			if cur_pipeline != pipeline {
				self.flush();
				return self.push_no_index(verts, pipeline, uniform);
			}
		} else {
			self.cur_pipeline = Some(pipeline.clone());
		}

		if let Some(cur_uniform) = &self.cur_uniform {
			if cur_uniform != uniform {
				self.flush();
				return self.push_no_index(verts, pipeline, uniform);
			}
		} else {
			self.cur_uniform = Some(uniform.clone());
		}

		if self.vqueue.len() + verts.len() >= self.vqueue.capacity() {
			self.flush();
			return self.push_no_index(verts, pipeline, uniform);
		}

		self.vqueue.extend_from_slice(&verts);

		return Ok(());

	}

	pub fn push(
		&mut self,
		verts: &[f32],
		indices: &[u32],
		pipeline: &Pipeline<V, U>,
		uniform: &U,
	) -> Result<()> {

		self.push_no_index(verts, pipeline, uniform)?;

		let iqueue = self.iqueue.as_mut().ok_or(Error::Gfx(format!("not initialized with index buffer")))?;

		if iqueue.len() + indices.len() >= iqueue.capacity() {
			self.flush();
			return self.push(verts, indices, pipeline, uniform);
		}

		let offset = (self.vqueue.len() / V::STRIDE) as u32;

		let indices = indices
			.iter()
			.map(|i| *i + offset)
			.collect::<Vec<u32>>();
			;

		iqueue.extend_from_slice(&indices);

		return Ok(());

	}

	pub fn push_shape<S: Shape>(
		&mut self,
		shape: S,
		pipeline: &Pipeline<V, U>,
		uniform: &U,
	) -> Result<()> {

		if let Some(indices) = S::indices() {
			self.push(&[], indices, pipeline, uniform)?;
		} else {
			self.push_no_index(&[], pipeline, uniform)?;
		}

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

		if let Some(ibuf) = &self.ibuf {
			if let Some(iqueue) = &self.iqueue {
				ibuf.data(0, &iqueue);
			}
		}

		let count = self.iqueue
			.as_ref()
			.map(|i| i.len()).unwrap_or(self.vqueue.len());

		pipeline.draw(
			#[cfg(feature="gl3")]
			Some(&self.vao),
			#[cfg(not(feature="gl3"))]
			Some(&self.vbuf),
			#[cfg(not(feature="gl3"))]
			self.ibuf.as_ref(),
			Some(uniform),
			count as u32,
			self.prim,
		);

		self.cur_pipeline = None;
		self.cur_uniform = None;
		self.vqueue.clear();
		self.iqueue.as_mut().map(|i| i.clear());
		self.draw_count += 1;

	}

	pub fn empty(&self) -> bool {
		return self.vqueue.is_empty();
	}

	pub fn clear(&mut self) {

		self.cur_pipeline = None;
		self.cur_uniform = None;
		self.vqueue.clear();
		self.iqueue.as_mut().map(|i| i.clear());
		self.draw_count = 0;

	}

	pub fn draw_count(&self) -> usize {
		return self.draw_count;
	}

	pub fn drop(&self) {

		self.vbuf.drop();
		self.ibuf.as_ref().map(|i| i.drop());
		#[cfg(feature="gl3")]
		self.vao.drop();

	}

}

