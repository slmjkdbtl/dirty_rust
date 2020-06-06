// wengwengweng

use super::*;

pub fn spline(pts: &[Vec2]) -> Vec<Vec2> {

	let mut spts = vec![];

	for i in 0..pts.len() - 1 {

		let p1 = pts[i];
		let p2 = pts[i + 1];
		let c1 = pts.get(i.wrapping_sub(1)).cloned().unwrap_or(p1);
		let c2 = pts.get(i + 2).cloned().unwrap_or(p2);

		let mut t = 0.0;

		while t <= 1.0 {

			let tt = t * t;
			let ttt = tt * t;

			// from OneLoneCoder
			let qc1 = -ttt + 2.0 * tt - t;
			let qp1 = 3.0 * ttt - 5.0 * tt + 2.0;
			let qp2 = -3.0 * ttt + 4.0 * tt + t;
			let qc2 = ttt - tt;

			let tx = 0.5 * (c1.x * qc1 + p1.x * qp1 + p2.x * qp2 + c2.x * qc2);
			let ty = 0.5 * (c1.y * qc1 + p1.y * qp1 + p2.y * qp2 + c2.y * qc2);

			spts.push(vec2!(tx, ty));
			t += 0.05;

		}

	}

	return spts;

}

pub fn spline_loop(pts: &[Vec2]) -> Vec<Vec2> {

	let mut spts = vec![];

	for i in 0..pts.len() {

		let p1 = pts[i];
		let p2 = pts[(i + 1) % pts.len()];

		let c1 = if i == 0 {
			pts[pts.len() - 1]
		} else {
			pts[i - 1]
		};

		let c2 = pts[(i + 2) % pts.len()];

		let mut t = 0.0;

		while t <= 1.0 {

			let tt = t * t;
			let ttt = tt * t;

			// from OneLoneCoder
			let qc1 = -ttt + 2.0 * tt - t;
			let qp1 = 3.0 * ttt - 5.0 * tt + 2.0;
			let qp2 = -3.0 * ttt + 4.0 * tt + t;
			let qc2 = ttt - tt;

			let tx = 0.5 * (c1.x * qc1 + p1.x * qp1 + p2.x * qp2 + c2.x * qc2);
			let ty = 0.5 * (c1.y * qc1 + p1.y * qp1 + p2.y * qp2 + c2.y * qc2);

			spts.push(vec2!(tx, ty));
			t += 0.05;

		}

	}

	return spts;

}

pub fn uv_quad(cols: usize, rows: usize) -> MeshData {

	let mut verts = vec![];
	let mut indices = vec![];

	let gw = 1.0 / cols as f32;
	let gh = 1.0 / rows as f32;
	let color = rgba!(1);
	let normal = vec3!(0, 0, 1);

	for i in 0..cols {

		for j in 0..rows {

			let x = gw * i as f32;
			let y = gh * j as f32;

			let index = i * cols + j;

			let ii = [0, 3, 1, 1, 3, 2]
				.iter()
				.map(|i| {
					return (i + index * 4) as u32;
				});

			indices.extend(ii);

			verts.push(Vertex {
				pos: vec3!(x, y + gh, 0) - vec3!(0.5, 0.5, 0),
				color,
				normal,
				uv: vec2!(x, y),
			});

			verts.push(Vertex {
				pos: vec3!(x + gw, y + gh, 0) - vec3!(0.5, 0.5, 0),
				color,
				normal,
				uv: vec2!(x + gw, y),
			});

			verts.push(Vertex {
				pos: vec3!(x + gw, y, 0) - vec3!(0.5, 0.5, 0),
				color,
				normal,
				uv: vec2!(x + gw, y + gh),
			});

			verts.push(Vertex {
				pos: vec3!(x, y, 0) - vec3!(0.5, 0.5, 0),
				color,
				normal,
				uv: vec2!(x, y + gh),
			});

		}

	}

	return MeshData {
		vertices: verts,
		indices,
	};

}

pub fn cube(s: f32) -> MeshData {

	let r = s * 0.5;

	let rect = MeshData {
		vertices: vec![
			Vertex {
				pos: vec3!(-r, -r, r),
				normal: vec3!(0, 1, 0),
				color: rgba!(1),
				uv: vec2!(0, 0),
			},
			Vertex {
				pos: vec3!(r, -r, r),
				normal: vec3!(0, 1, 0),
				color: rgba!(1),
				uv: vec2!(0, 0),
			},
			Vertex {
				pos: vec3!(r, -r, -r),
				normal: vec3!(0, 1, 0),
				color: rgba!(1),
				uv: vec2!(0, 0),
			},
			Vertex {
				pos: vec3!(-r, -r, -r),
				normal: vec3!(0, 1, 0),
				color: rgba!(1),
				uv: vec2!(0, 0),
			},
		],
		indices: vec![0, 1, 2, 0, 2, 3],
	};

	return ops::extrude(&rect, &[(0, 1), (1, 2), (2, 3), (3, 1)], s);

}

// TODO
pub fn sphere(r: f32) -> MeshData {

	let verts = vec![];
	let indices = vec![];

	return MeshData {
		vertices: verts,
		indices,
	};

}

// TODO
pub fn cylinder(r: f32, h: f32, s: usize) -> Option<MeshData> {

	let mut verts = vec![];
	let mut edges = vec![];
	let mut pts = vec![];

	for i in 0..s {

		let a = f32::to_radians(360.0) / s as f32 * i as f32;
		let p = Vec2::from_angle(a) * r;

		pts.push(p);

		verts.push(Vertex {
			pos: vec3!(p.x, 0.0, p.y),
			normal: vec3!(0, 1, 0),
			color: rgba!(1),
			uv: vec2!(0, 0),
		});

	}

	let tri = ops::triangulate(&pts)?;

	let indices = tri.triangles.iter().map(|i| {
		return *i as u32;
	}).collect();

	for i in 0..tri.hull.len() {

		let i1 = tri.hull[i];
		let i2 = tri.hull[(i + 1) % tri.hull.len()];

		edges.push((i1 as u32, i2 as u32));

	}

	let circle = MeshData {
		vertices: verts,
		indices,
	};

	return Some(ops::extrude(&circle, &edges, h));

}

// TODO
pub fn torus(r1: f32, r2: f32) -> MeshData {

	let verts = vec![];
	let indices = vec![];

	return MeshData {
		vertices: verts,
		indices,
	};

}

pub fn checkerboard(s: f32, c: usize, r: usize) -> MeshData {

	let mut verts = vec![];
	let mut indices = vec![];

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
				color,
				uv: vec2!(0, 0),
			});

			verts.push(Vertex {
				pos: pt + vec3!(s, 0, 0),
				normal: vec3!(0, 1, 0),
				color,
				uv: vec2!(0, 0),
			});

			verts.push(Vertex {
				pos: pt + vec3!(s, 0, s),
				normal: vec3!(0, 1, 0),
				color,
				uv: vec2!(0, 0),
			});

			verts.push(Vertex {
				pos: pt + vec3!(0, 0, s),
				normal: vec3!(0, 1, 0),
				color,
				uv: vec2!(0, 0),
			});

			let start = (i * c + j) as u32 * 4;
			let tl = 0 + start;
			let tr = 1 + start;
			let br = 2 + start;
			let bl = 3 + start;

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

	return MeshData {
		vertices: verts,
		indices,
	};

}

