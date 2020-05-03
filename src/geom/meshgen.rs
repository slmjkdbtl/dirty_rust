// wengwengweng

use crate::math::*;
use crate::gfx::MeshData;
use crate::gfx::Vertex;

pub fn checkerboard(s: f32, c: usize, r: usize) -> (MeshData, Vec<(u32, u32)>) {

	let mut verts = vec![];
	let mut indices = vec![];
	let mut edges = vec![];

	let w = s * c as f32;
	let h = s * r as f32;

	let p0 = vec3!(-w / 2.0, 0, -h / 2.0);
	let mut b = false;

	for i in 0..r {

		for j in 0..c {

			b = !b;

			let pt = p0 + vec3!(s * i as f32, 0, s * j as f32);

			let color = if b {
				rgba!(0.5, 0.5, 0.5, 1)
			} else {
				rgba!(0.75, 0.75, 0.75, 1)
			};

			verts.push(Vertex {
				pos: pt + vec3!(0),
				normal: vec3!(0, 1, 0),
				color: color,
				uv: vec2!(0, 0),
			});

			verts.push(Vertex {
				pos: pt + vec3!(s, 0, 0),
				normal: vec3!(0, 1, 0),
				color: color,
				uv: vec2!(0, 0),
			});

			verts.push(Vertex {
				pos: pt + vec3!(s, 0, s),
				normal: vec3!(0, 1, 0),
				color: color,
				uv: vec2!(0, 0),
			});

			verts.push(Vertex {
				pos: pt + vec3!(0, 0, s),
				normal: vec3!(0, 1, 0),
				color: color,
				uv: vec2!(0, 0),
			});

			let start = (i * c + j) as u32 * 4;
			let tl = 0 + start;
			let tr = 1 + start;
			let br = 2 + start;
			let bl = 3 + start;

			if i == 0 {
				edges.push((tl, bl));
			}

			if i == r - 1 {
				edges.push((tr, br));
			}

			if j == 0 {
				edges.push((tl, tr));
			}

			if j == c - 1 {
				edges.push((bl, br));
			}

			indices.extend_from_slice(&[
				tl,
				br,
				tr,
				tl,
				bl,
				br
			]);

		}

	}

	return (MeshData {
		vertices: verts,
		indices: indices,
	}, edges);

}

