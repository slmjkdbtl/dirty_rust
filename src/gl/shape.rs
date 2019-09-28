// wengwengweng

use super::*;

pub trait Shape {

	type Vertex: VertexLayout;
	const COUNT: usize;
	fn vertices(&self, queue: &mut Vec<f32>);
	fn indices() -> &'static [u32];

}

