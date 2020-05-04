// wengwengweng

use super::*;

pub trait Shape {
	type Vertex: VertexLayout;
	const COUNT: usize;
	fn vertices(&self) -> Vec<Self::Vertex>;
	fn indices() -> &'static [u32];
}

