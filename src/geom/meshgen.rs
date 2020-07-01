// wengwengweng

use std::f32::consts::PI;

use super::*;
use std::ops::*;

const SPLINE_RES: f32 = 0.05;

// TODO: trait alias
fn spline_interp<T>(p1: T, p2: T, c1: T, c2: T) -> Vec<T>
	where T:
		Copy
		+ Add<Output = T>
		+ Mul<f32, Output = T>
{

	let mut pts = Vec::with_capacity((1.0 / SPLINE_RES) as usize);
	let mut t = 0.0;

	while t <= 1.0 {

		let tt = t * t;
		let ttt = tt * t;

		// from OneLoneCoder
		let qc1 = -ttt + 2.0 * tt - t;
		let qp1 = 3.0 * ttt - 5.0 * tt + 2.0;
		let qp2 = -3.0 * ttt + 4.0 * tt + t;
		let qc2 = ttt - tt;
		let pt = (c1 * qc1 + p1 * qp1 + p2 * qp2 + c2 * qc2) * 0.5;

		pts.push(pt);
		t += SPLINE_RES;

	}

	return pts;

}

pub fn spline<T>(pts: &[T]) -> Vec<T>
	where T:
		Copy
		+ Add<Output = T>
		+ Mul<f32, Output = T>
{

	// TODO: calculate the capacity
	let mut spts = Vec::with_capacity((1.0 / SPLINE_RES) as usize * pts.len());

	for i in 0..pts.len() - 1 {

		let p1 = pts[i];
		let p2 = pts[i + 1];
		let c1 = pts.get(i.wrapping_sub(1)).cloned().unwrap_or(p1);
		let c2 = pts.get(i + 2).cloned().unwrap_or(p2);

		spts.append(&mut spline_interp(p1, p2, c1, c2));

	}

	return spts;

}

pub fn spline_loop<T>(pts: &[T]) -> Vec<T>
	where T:
		Copy
		+ Add<Output = T>
		+ Mul<f32, Output = T>
{

	// TODO: calculate the capacity
	let mut spts = Vec::with_capacity((1.0 / SPLINE_RES) as usize * pts.len());

	for i in 0..pts.len() {

		let p1 = pts[i];
		let p2 = pts[(i + 1) % pts.len()];

		let c1 = if i == 0 {
			pts[pts.len() - 1]
		} else {
			pts[i - 1]
		};

		let c2 = pts[(i + 2) % pts.len()];

		spts.append(&mut spline_interp(p1, p2, c1, c2));

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

	let vertices = vec![
		// front
		Vertex {
			pos: vec3!(-r, -r, r),
			normal: vec3!(0, 0, 1),
			color: rgba!(1),
			uv: vec2!(0, 0),
		},
		Vertex {
			pos: vec3!(r, -r, r),
			normal: vec3!(0, 0, 1),
			color: rgba!(1),
			uv: vec2!(1, 0),
		},
		Vertex {
			pos: vec3!(r, r, r),
			normal: vec3!(0, 0, 1),
			color: rgba!(1),
			uv: vec2!(1, 1),
		},
		Vertex {
			pos: vec3!(-r, r, r),
			normal: vec3!(0, 0, 1),
			color: rgba!(1),
			uv: vec2!(0, 1),
		},
		// back
		Vertex {
			pos: vec3!(-r, -r, -r),
			normal: vec3!(0, 0, -1),
			color: rgba!(1),
			uv: vec2!(0, 0),
		},
		Vertex {
			pos: vec3!(-r, r, -r),
			normal: vec3!(0, 0, -1),
			color: rgba!(1),
			uv: vec2!(1, 0),
		},
		Vertex {
			pos: vec3!(r, r, -r),
			normal: vec3!(0, 0, -1),
			color: rgba!(1),
			uv: vec2!(1, 1),
		},
		Vertex {
			pos: vec3!(r, -r, -r),
			normal: vec3!(0, 0, -1),
			color: rgba!(1),
			uv: vec2!(0, 1),
		},
		// top
		Vertex {
			pos: vec3!(-r, r, -r),
			normal: vec3!(0, 1, 0),
			color: rgba!(1),
			uv: vec2!(0, 0),
		},
		Vertex {
			pos: vec3!(-r, r, r),
			normal: vec3!(0, 1, 0),
			color: rgba!(1),
			uv: vec2!(1, 0),
		},
		Vertex {
			pos: vec3!(r, r, r),
			normal: vec3!(0, 1, 0),
			color: rgba!(1),
			uv: vec2!(1, 1),
		},
		Vertex {
			pos: vec3!(r, r, -r),
			normal: vec3!(0, 1, 0),
			color: rgba!(1),
			uv: vec2!(0, 1),
		},
		// bottom
		Vertex {
			pos: vec3!(-r, -r, -r),
			normal: vec3!(0, -1, 0),
			color: rgba!(1),
			uv: vec2!(0, 0),
		},
		Vertex {
			pos: vec3!(r, -r, -r),
			normal: vec3!(0, -1, 0),
			color: rgba!(1),
			uv: vec2!(1, 0),
		},
		Vertex {
			pos: vec3!(r, -r, r),
			normal: vec3!(0, -1, 0),
			color: rgba!(1),
			uv: vec2!(1, 1),
		},
		Vertex {
			pos: vec3!(-r, -r, r),
			normal: vec3!(0, -1, 0),
			color: rgba!(1),
			uv: vec2!(0, 1),
		},
		// right
		Vertex {
			pos: vec3!(r, -r, -r),
			normal: vec3!(1, 0, 0),
			color: rgba!(1),
			uv: vec2!(0, 0),
		},
		Vertex {
			pos: vec3!(r, r, -r),
			normal: vec3!(1, 0, 0),
			color: rgba!(1),
			uv: vec2!(1, 0),
		},
		Vertex {
			pos: vec3!(r, r, r),
			normal: vec3!(1, 0, 0),
			color: rgba!(1),
			uv: vec2!(1, 1),
		},
		Vertex {
			pos: vec3!(r, -r, r),
			normal: vec3!(1, 0, 0),
			color: rgba!(1),
			uv: vec2!(0, 1),
		},
		// left
		Vertex {
			pos: vec3!(-r, -r, -r),
			normal: vec3!(-1, 0, 0),
			color: rgba!(1),
			uv: vec2!(0, 0),
		},
		Vertex {
			pos: vec3!(-r, -r, r),
			normal: vec3!(-1, 0, 0),
			color: rgba!(1),
			uv: vec2!(1, 0),
		},
		Vertex {
			pos: vec3!(-r, r, r),
			normal: vec3!(-1, 0, 0),
			color: rgba!(1),
			uv: vec2!(1, 1),
		},
		Vertex {
			pos: vec3!(-r, r, -r),
			normal: vec3!(-1, 0, 0),
			color: rgba!(1),
			uv: vec2!(0, 1),
		},
	];

	let indices = vec![
		0, 1, 2, 0, 2, 3, // front
		4, 5, 6, 4, 6, 7, // back
		8, 9, 10, 8, 10, 11, // top
		12, 13, 14, 12, 14, 15, // bottom
		16, 17, 18, 16, 18, 19, // right
		20, 21, 22, 20, 22, 23 // left
	];

	return MeshData {
		vertices: vertices,
		indices: indices,
	};

}

pub fn sphere(r: f32, rx: usize, ry: usize) -> MeshData {

	let mut verts = vec![];
	let mut indices = vec![];

	for i in 0..=ry {

		let t = i as f32 * PI / ry as f32;
		let t_sin = t.sin();
		let t_cos = t.cos();

		for j in 0..=rx {

			let p = j as f32 * 2.0 * PI / rx as f32;
			let p_sin = p.sin();
			let p_cos = p.cos();

			let x = p_cos * t_sin;
			let y = t_cos;
			let z = p_sin * t_sin;

			let u = 1.0 - (j as f32 / rx as f32);
			let v = 1.0 - (i as f32 / ry as f32);

			verts.push(Vertex {
				pos: vec3!(x, y, z) * r,
				normal: vec3!(x, y, z),
				color: rgba!(1),
				uv: vec2!(u, v),
			});

			if i < ry && j < rx {

				let first = i * (rx + 1) + j;
				let second = first + rx + 1;
				let first = first as u32;
				let second = second as u32;

				indices.push(first);
				indices.push(second);
				indices.push(first + 1);
				indices.push(second);
				indices.push(second + 1);
				indices.push(first + 1);

			}

		}

	}

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

