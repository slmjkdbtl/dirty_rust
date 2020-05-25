// wengwengweng

use super::*;

pub use delaunay::triangulate;

pub fn extrude(data: &MeshData, edges: &[(u32, u32)], dis: f32) -> MeshData {

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

	for (i1, i2) in edges {
		indices.push(*i1);
		indices.push(*i2);
		indices.push(*i1 + data.vertices.len() as u32);
		indices.push(*i1 + data.vertices.len() as u32);
		indices.push(*i2 + data.vertices.len() as u32);
		indices.push(*i2);
	}

	return MeshData {
		vertices: verts,
		indices,
	};

}

pub fn gen_normals(pos: &[Vec3], indices: &[u32]) -> Vec<Vec3> {

	let vert_count = pos.len();
	let mut normals = vec![vec3!(0); vert_count];

	indices
		.chunks(3)
		.for_each(|tri| {

			let i1 = tri[0] as usize;
			let i2 = tri[1] as usize;
			let i3 = tri[2] as usize;
			let v1 = pos[i1];
			let v2 = pos[i2];
			let v3 = pos[i3];
			let normal = Vec3::cross((v2 - v1), (v3 - v1));

			normals[i1] += normal;
			normals[i2] += normal;
			normals[i3] += normal;

		});

	return normals
		.into_iter()
		.map(|p| p.unit())
		.collect();

}

