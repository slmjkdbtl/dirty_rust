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
		indices: indices,
	};

}

pub fn hull(pts: &[Vec2]) -> Vec<usize> {

	use std::cmp::*;

	let mut hull = vec![];
	let mut pts = pts.to_vec();

	pts.sort_by(|p1, p2| {
		return PartialOrd::partial_cmp(&p1.x, &p2.x)
			.unwrap_or(Ordering::Equal);
	});

	let left = match pts.get(0) {
		Some(pt) => pt,
		None => return vec![],
	};

	return hull;

}

