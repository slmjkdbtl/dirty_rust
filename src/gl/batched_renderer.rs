// wengwengweng

use super::*;
use crate::Result;

// TODO: shouldn't need to bind to one type of UniformInterface
pub struct BatchedRenderer<V: VertexLayout, U: UniformInterface + PartialEq + Clone> {

	ctx: Rc<GLCtx>,
	vbuf: VertexBuffer<V>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	vqueue: Vec<f32>,
	iqueue: Vec<u32>,
	prim: Primitive,
	cur_uniform: Option<U>,
	cur_program: Option<Program<U>>,
	cur_fbuf: Option<Framebuffer>,
	draw_count: usize,

}

impl<V: VertexLayout, U: UniformInterface + PartialEq + Clone> BatchedRenderer<V, U> {

	pub fn new(device: &Device, max_vertices: usize, max_indices: usize) -> Result<Self> {

		let vbuf = VertexBuffer::new(&device, max_vertices, BufferUsage::Dynamic)?;
		let ibuf = IndexBuffer::new(&device, max_indices, BufferUsage::Dynamic)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::init(&device, &vbuf)?;

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
			cur_program: None,
			cur_fbuf: None,
			draw_count: 0,
		});

	}

	pub fn push(
		&mut self,
		verts: &[f32],
		indices: &[u32],
		program: &Program<U>,
		uniform: &U,
		fbuf: Option<&Framebuffer>,
	) -> Result<()> {

		let mut flushed = false;

		if let Some(cur_program) = &self.cur_program {
			if cur_program != program {
				if !flushed {
					self.flush();
					flushed = true;
				}
				self.cur_program = Some(program.clone());
			}
		} else {
			self.cur_program = Some(program.clone());
		}

		if let Some(cur_uniform) = &self.cur_uniform {
			if cur_uniform != uniform {
				if !flushed {
					self.flush();
					flushed = true;
				}
				self.cur_uniform = Some(uniform.clone());
				self.cur_program = Some(program.clone());
			}
		} else {
			self.cur_uniform = Some(uniform.clone());
		}

		if self.cur_fbuf.as_ref() != fbuf {
			if !flushed {
				self.flush();
				flushed = true;
			}
			self.cur_fbuf = fbuf.map(Clone::clone);
			self.cur_uniform = Some(uniform.clone());
			self.cur_program = Some(program.clone());
		}

		let offset = (self.vqueue.len() / V::STRIDE) as u32;

		let indices = indices
			.iter()
			.map(|i| *i + offset)
			.collect::<Vec<u32>>();
			;

		if self.vqueue.len() + verts.len() >= self.vqueue.capacity() {
			self.vqueue.clear();
			return Err(Error::Gfx("max draw count".into()));
		}

		if self.iqueue.len() + indices.len() >= self.iqueue.capacity() {
			self.iqueue.clear();
			return Err(Error::Gfx("max draw count reached".into()));
		}

		self.vqueue.extend_from_slice(&verts);
		self.iqueue.extend_from_slice(&indices);

		return Ok(());

	}

	pub fn push_shape<S: Shape>(
		&mut self,
		shape: S,
		program: &Program<U>,
		uniform: &U,
		fbuf: Option<&Framebuffer>,
	) -> Result<()> {

		self.push(&[], &S::indices(), program, uniform, fbuf)?;
		shape.push(&mut self.vqueue);

		return Ok(());

	}

	pub fn flush(&mut self) {

		if self.empty() {
			return;
		}

		let program = match &self.cur_program {
			Some(p) => p,
			None => return,
		};

		let uniform = match &self.cur_uniform {
			Some(p) => p,
			None => return,
		};

		self.vbuf.data(0, &self.vqueue);
		self.ibuf.data(0, &self.iqueue);

		draw(
			&self.ctx,
			#[cfg(feature="gl3")]
			&self.vao,
			#[cfg(not(feature="gl3"))]
			&self.vbuf,
			&self.ibuf,
			&program,
			uniform,
			self.cur_fbuf.as_ref(),
			self.iqueue.len() as u32,
			self.prim,
		);

		self.cur_program = None;
		self.cur_uniform = None;
		self.cur_fbuf = None;
		self.vqueue.clear();
		self.iqueue.clear();
		self.draw_count += 1;

	}

	pub fn empty(&self) -> bool {
		return self.vqueue.is_empty();
	}

	pub fn clear(&mut self) {

		self.cur_program = None;
		self.cur_uniform = None;
		self.cur_fbuf = None;
		self.vqueue.clear();
		self.iqueue.clear();
		self.draw_count = 0;

	}

	pub fn draw_count(&self) -> usize {
		return self.draw_count;
	}

}

