// wengwengweng

use super::*;

/// Mesh Data with Vertices & Indices
#[derive(Clone, Serialize, Deserialize)]
pub struct MeshData {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,
}

/// A Buffered GPU Mesh
#[derive(Clone, PartialEq)]
pub struct Mesh {
	vbuf: VertexBuffer<Vertex>,
	ibuf: IndexBuffer,
	count: usize,
}

impl Mesh {

	/// create a mesh from vertices and indices
	pub fn new(ctx: &impl GLCtx, verts: &[Vertex], indices: &[u32]) -> Result<Self> {

		let vbuf = VertexBuffer::<Vertex>::from(ctx, &verts)?;
		let ibuf = IndexBuffer::from(ctx, &indices)?;

		return Ok(Self {
			vbuf,
			ibuf,
			count: indices.len(),
		});

	}

	/// create a mesh from [`MeshData`](struct.MeshData.html)
	pub fn from_meshdata(ctx: &impl GLCtx, data: &MeshData) -> Result<Self> {
		return Self::new(ctx, &data.vertices, &data.indices);
	}

	pub(super) fn vbuf(&self) -> &VertexBuffer<Vertex> {
		return &self.vbuf;
	}

	pub(super) fn ibuf(&self) -> &IndexBuffer {
		return &self.ibuf;
	}

	pub(super) fn count(&self) -> usize {
		return self.count;
	}

}

