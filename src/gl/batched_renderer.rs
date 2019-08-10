// wengwengweng

use glow::Context;

use super::*;
use crate::Result;

type GLCtx = glow::native::Context;

pub trait Shape {

	type Vertex: VertexLayout;
	const COUNT: usize;
	fn push(&self, queue: &mut Vec<f32>);
	fn indices() -> Vec<u32>;

}

pub struct BatchedRenderer<S: Shape> {

	ctx: Rc<GLCtx>,
	vbuf: VertexBuffer<S::Vertex>,
	ibuf: IndexBuffer,
	#[cfg(feature="gl3")]
	vao: VertexArray,
	queue: Vec<f32>,
	mode: DrawMode,
	cur_texture: Option<Texture>,
	cur_program: Option<Program>,
	draw_count: usize,
	shape: PhantomData<S>,

}

impl<S: Shape> BatchedRenderer<S> {

	pub fn new(device: &Device, max: usize) -> Result<Self> {

		let indices = S::indices();
		let vert_count = S::COUNT;
		let vert_stride = S::Vertex::STRIDE;
		let max_vertices = max * vert_stride * vert_count;
		let max_indices = max * indices.len();

		let indices_batch: Vec<u32> = indices
			.iter()
			.cycle()
			.take(max_indices)
			.enumerate()
			.map(|(i, vertex)| vertex + i as u32 / 6 * 4)
			.collect();

		let vbuf = VertexBuffer::new(&device, vert_count * vert_stride * max, BufferUsage::Dynamic)?;
		let ibuf = IndexBuffer::init(&device, &indices_batch)?;

		#[cfg(feature="gl3")]
		let vao = VertexArray::init(&device, &vbuf)?;

		return Ok(Self {
			ctx: device.ctx.clone(),
			vbuf: vbuf,
			ibuf: ibuf,
			#[cfg(feature="gl3")]
			vao: vao,
			queue: Vec::with_capacity(max_vertices),
			mode: DrawMode::Triangles,
			cur_texture: None,
			cur_program: None,
			draw_count: 0,
			shape: PhantomData,
		});

	}

	pub fn push(&mut self, shape: S, program: &Program, otex: Option<&Texture>) -> Result<()> {

		if let Some(tex) = otex {
			if let Some(cur_tex) = &self.cur_texture {
				if cur_tex != tex {
					self.flush();
					self.cur_texture = Some(tex.clone());
				}
			} else {
				self.flush();
				self.cur_texture = Some(tex.clone());
			}
		}

		if let Some(cur_program) = &self.cur_program {
			if cur_program != program {
				self.flush();
				self.cur_program = Some(program.clone());
			}
		} else {
			self.cur_program = Some(program.clone());
		}

		if self.queue.len() >= self.queue.capacity() {
			self.queue.clear();
			return Err(Error::MaxDraw);
		}

		shape.push(&mut self.queue);

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

		self.vbuf.data(0, &self.queue);

		if let Some(tex) = &self.cur_texture {
			tex.bind();
		}

		draw(
			&self.ctx,
			#[cfg(feature="gl3")]
			&self.vao,
			#[cfg(not(feature="gl3"))]
			&self.vbuf,
			&self.ibuf,
			&program,
			(self.queue.len() * S::indices().len() / S::Vertex::STRIDE / S::COUNT) as u32,
			self.mode,
		);

		if let Some(tex) = &self.cur_texture {
			tex.unbind();
		}

		self.cur_texture = None;
		self.cur_program = None;
		self.queue.clear();
		self.draw_count += 1;

	}

	pub fn empty(&self) -> bool {
		return self.queue.is_empty();
	}

	pub fn frame_end(&mut self) {
		self.flush();
	}

	pub fn clear(&mut self) {

		self.cur_texture = None;
		self.cur_program = None;
		self.queue.clear();
		self.draw_count = 0;

	}

	pub fn draw_count(&self) -> usize {
		return self.draw_count;
	}

}

