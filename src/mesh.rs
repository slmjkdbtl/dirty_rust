// wengwengweng

use crate::gfx::*;

pub fn extrude(data: &MeshData, dis: f32) -> MeshData {

	let mut verts = data.vertices.to_vec();

	for v in &data.vertices {
		verts.push(Vertex {
			pos: v.pos + v.normal * dis,
			normal: -v.normal,
			color: v.color,
			uv: v.uv,
		});
	}

	let mut indices = data.indices.to_vec();

	indices.append(&mut data.indices
		.iter()
		.map(|i| *i + data.vertices.len() as u32)
		.collect::<Vec<u32>>()
	);

	// TODO: overlapping vertices
	// TODO: don't connect verts that's inside? too complicated
	for i in 0..data.indices.len() {

		let i1 = data.indices[i];
		let i2 = data.indices[(i + 1) % data.indices.len()];

		indices.push(i1);
		indices.push(i2);
		indices.push(i1 + data.vertices.len() as u32);
		indices.push(i1 + data.vertices.len() as u32);
		indices.push(i2 + data.vertices.len() as u32);
		indices.push(i2);

	}

	return MeshData {
		vertices: verts,
		indices: indices,
	};

}

