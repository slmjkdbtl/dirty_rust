// wengwengweng

use super::*;

pub(super) fn bind_attrs<V: VertexLayout>(gl: &glow::Context) {

	unsafe {

		let mut offset = 0;

		for (i, (name, size)) in V::attrs().into_iter().enumerate() {

			gl.vertex_attrib_pointer_f32(
				i as u32,
				*size as i32,
				glow::FLOAT,
				false,
				mem::size_of::<V>() as i32,
				(offset * mem::size_of::<f32>()) as i32,
			);

			gl.enable_vertex_attrib_array(i as u32);
			offset += size;

		}

	}

}

pub(super) trait VertexLayout: Clone {
	fn attrs() -> &'static[(&'static str, usize)];
}

#[derive(Clone)]
pub(super) struct VertexBuffer<V: VertexLayout> {
	handle: Rc<BufferHandle>,
	gl: Rc<glow::Context>,
	_layout: PhantomData<V>,
}

impl<V: VertexLayout> VertexBuffer<V> {

	pub fn new(ctx: &impl GLCtx, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let handle = BufferHandle::new(gl.clone())?;

			let buf = Self {
				handle: Rc::new(handle),
				gl: gl,
				_layout: PhantomData,
			};

			buf.bind();

			buf.gl.buffer_data_size(
				glow::ARRAY_BUFFER,
				(count * mem::size_of::<V>()) as i32,
				usage.as_glow(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn from(ctx: &impl GLCtx, data: &[V]) -> Result<Self> {

		let buf = Self::new(ctx, data.len(), BufferUsage::Static)?;
		buf.data(0, data);
		return Ok(buf);

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.handle.id()));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
		}
	}

	pub fn data(&self, offset: usize, data: &[V]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.gl.buffer_sub_data_u8_slice(
				glow::ARRAY_BUFFER,
				(offset * mem::size_of::<V>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

}

impl<V: VertexLayout> PartialEq for VertexBuffer<V> {
	fn eq(&self, other: &Self) -> bool {
		return self.handle == other.handle;
	}
}

#[derive(Clone)]
pub(super) struct IndexBuffer {
	handle: Rc<BufferHandle>,
	gl: Rc<glow::Context>,
}

impl IndexBuffer {

	pub fn new(ctx: &impl GLCtx, count: usize, usage: BufferUsage) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let handle = BufferHandle::new(gl.clone())?;

			let buf = Self {
				handle: Rc::new(handle),
				gl: gl,
			};

			buf.bind();

			buf.gl.buffer_data_size(
				glow::ELEMENT_ARRAY_BUFFER,
				(count * mem::size_of::<u32>()) as i32,
				usage.as_glow(),
			);

			buf.unbind();

			return Ok(buf);

		}

	}

	pub fn from(ctx: &impl GLCtx, data: &[u32]) -> Result<Self> {

		let buf = Self::new(ctx, data.len(), BufferUsage::Static)?;
		buf.data(0, data);
		return Ok(buf);

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.handle.id()));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
		}
	}

	pub fn data(&self, offset: usize, data: &[u32]) {

		unsafe {

			let byte_len = mem::size_of_val(data) / mem::size_of::<u8>();
			let byte_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len);

			self.bind();

			self.gl.buffer_sub_data_u8_slice(
				glow::ELEMENT_ARRAY_BUFFER,
				(offset * mem::size_of::<u32>()) as i32,
				byte_slice,
			);

			self.unbind();

		}

	}

}

impl PartialEq for IndexBuffer {
	fn eq(&self, other: &Self) -> bool {
		return self.handle == other.handle;
	}
}

