// wengwengweng

use crate::gfx::*;

pub fn extrude(verts: &[Vertex], indices: &[u32], dis: f32) -> MeshData {

	let mut nverts = verts.to_vec();

	for v in verts {
		nverts.push(Vertex {
			pos: v.pos + v.normal * dis,
			normal: -v.normal,
			color: v.color,
			uv: v.uv,
		});
	}

	let mut nindices = indices.to_vec();

	nindices.append(&mut indices
		.iter()
		.map(|i| *i + verts.len() as u32)
		.collect::<Vec<u32>>()
	);

	for i in 0..indices.len() {

		let i1 = indices[i];
		let i2 = indices[(i + 1) % indices.len()];

		nindices.push(i1);
		nindices.push(i2);
		nindices.push(i1 + verts.len() as u32);
		nindices.push(i1 + verts.len() as u32);
		nindices.push(i2 + verts.len() as u32);
		nindices.push(i2);

	}

	return MeshData {
		vertices: nverts,
		indices: nindices,
	};

}

