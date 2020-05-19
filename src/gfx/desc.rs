// wengwengweng

use std::mem;

use serde::Serialize;
use serde::Deserialize;

use crate::*;
use math::*;
use gfx::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct MeshData {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Vertex {
	pub pos: Vec3,
	pub uv: Vec2,
	pub normal: Vec3,
	pub color: Color,
}

impl gl::VertexLayout for Vertex {
	fn attrs() -> gl::VertexAttrGroup {
		return &[
			("a_pos", 3),
			("a_uv", 2),
			("a_normal", 3),
			("a_color", 4),
		];
	}
}

#[derive(Clone, PartialEq)]
pub(crate) struct Uniform {
	pub proj: Mat4,
	pub view: Mat4,
	pub model: Mat4,
	pub color: Color,
	pub tex: Texture,
	pub custom: Option<Vec<(&'static str, gl::UniformValue)>>,
}

impl gl::UniformLayout for Uniform {

	fn values(&self) -> gl::UniformValues {

		let mut values: gl::UniformValues = hmap![
			"u_proj" => &self.proj,
			"u_view" => &self.view,
			"u_model" => &self.model,
			"u_color" => &self.color,
		];

		if let Some(custom) = &self.custom {
			for (name, v) in custom {
				values.insert(name, v);
			}
		}

		return values;

	}

	fn textures(&self) -> Vec<&dyn gl::Texture> {
		return vec![self.tex.gl_tex()];
	}

}

pub(crate) struct QuadShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub flip: Flip,
}

impl gl::Shape for QuadShape {

	type Vertex = Vertex;
	const COUNT: usize = 4;

	fn vertices(&self) -> Vec<Self::Vertex> {

		let t = self.transform;
		let q = self.quad;
		let c = self.color;

		let p1 = t * vec3!(-0.5, 0.5, 0.0);
		let p2 = t * vec3!(0.5, 0.5, 0.0);
		let p3 = t * vec3!(0.5, -0.5, 0.0);
		let p4 = t * vec3!(-0.5, -0.5, 0.0);

		// TODO: flip img instead of tex coord
		let mut u1 = vec2!(q.x, q.y);
		let mut u2 = vec2!(q.x + q.w, q.y);
		let mut u3 = vec2!(q.x + q.w, q.y + q.h);
		let mut u4 = vec2!(q.x, q.y + q.h);

// 		let mut u1 = vec2!(q.x, q.y + q.h);
// 		let mut u2 = vec2!(q.x + q.w, q.y + q.h);
// 		let mut u3 = vec2!(q.x + q.w, q.y);
// 		let mut u4 = vec2!(q.x, q.y);

		match self.flip {
			Flip::X => {
				mem::swap(&mut u1, &mut u2);
				mem::swap(&mut u3, &mut u4);
			},
			Flip::Y => {
				mem::swap(&mut u1, &mut u4);
				mem::swap(&mut u2, &mut u3);
			},
			Flip::XY => {
				mem::swap(&mut u1, &mut u3);
				mem::swap(&mut u2, &mut u4);
			},
			_ => {},
		}

		return vec![
			Vertex {
				pos: p1,
				uv: u1,
				normal: vec3!(0, 0, 1),
				color: c
			},
			Vertex {
				pos: p2,
				uv: u2,
				normal: vec3!(0, 0, 1),
				color: c
			},
			Vertex {
				pos: p3,
				uv: u3,
				normal: vec3!(0, 0, 1),
				color: c
			},
			Vertex {
				pos: p4,
				uv: u4,
				normal: vec3!(0, 0, 1),
				color: c
			},
		];

	}

	fn indices() -> &'static [u32] {
		return &[0, 3, 1, 1, 3, 2];
	}

}

pub(crate) struct CubeShape;

impl gl::Shape for CubeShape {

	type Vertex = Vertex;
	const COUNT: usize = 24;

	fn vertices(&self) -> Vec<Self::Vertex> {

		let pos = [
			vec3!(-1, -1, 1),
			vec3!(-1, 1, 1),
			vec3!(-1, 1, -1),
			vec3!(-1, -1, -1),
			vec3!(-1, -1, -1),
			vec3!(-1, 1, -1),
			vec3!(1, 1, -1),
			vec3!(1, -1, -1),
			vec3!(1, -1, -1),
			vec3!(1, 1, -1),
			vec3!(1, 1, 1),
			vec3!(1, -1, 1),
			vec3!(1, -1, 1),
			vec3!(1, 1, 1),
			vec3!(-1, 1, 1),
			vec3!(-1, -1, 1),
			vec3!(-1, -1, -1),
			vec3!(1, -1, -1),
			vec3!(1, -1, 1),
			vec3!(-1, -1, 1),
			vec3!(1, 1, -1),
			vec3!(-1, 1, -1),
			vec3!(-1, 1, 1),
			vec3!(1, 1, 1),
		];

		let normals = [
			vec3!(-1, 0, 0),
			vec3!(-1, 0, 0),
			vec3!(-1, 0, 0),
			vec3!(-1, 0, 0),
			vec3!(0, 0, -1),
			vec3!(0, 0, -1),
			vec3!(0, 0, -1),
			vec3!(0, 0, -1),
			vec3!(1, 0, 0),
			vec3!(1, 0, 0),
			vec3!(1, 0, 0),
			vec3!(1, 0, 0),
			vec3!(0, 0, 1),
			vec3!(0, 0, 1),
			vec3!(0, 0, 1),
			vec3!(0, 0, 1),
			vec3!(0, -1, 0),
			vec3!(0, -1, 0),
			vec3!(0, -1, 0),
			vec3!(0, -1, 0),
			vec3!(0, 1, 0),
			vec3!(0, 1, 0),
			vec3!(0, 1, 0),
			vec3!(0, 1, 0),
		];

		let colors = [
			rgba!(0, 0, 1, 1),
			rgba!(0, 1, 1, 1),
			rgba!(0, 1, 0, 1),
			rgba!(1, 1, 1, 1),
			rgba!(1, 1, 1, 1),
			rgba!(0, 1, 0, 1),
			rgba!(1, 1, 0, 1),
			rgba!(1, 0, 0, 1),
			rgba!(1, 0, 0, 1),
			rgba!(1, 1, 0, 1),
			rgba!(1, 1, 1, 1),
			rgba!(1, 0, 1, 1),
			rgba!(1, 0, 1, 1),
			rgba!(1, 1, 1, 1),
			rgba!(0, 1, 1, 1),
			rgba!(0, 0, 1, 1),
			rgba!(1, 1, 1, 1),
			rgba!(1, 0, 0, 1),
			rgba!(1, 0, 1, 1),
			rgba!(0, 0, 1, 1),
			rgba!(1, 1, 0, 1),
			rgba!(0, 1, 0, 1),
			rgba!(0, 1, 1, 1),
			rgba!(1, 1, 1, 1),
		];

		return pos
			.iter()
			.zip(&normals)
			.zip(&colors)
			// zoop
			.map(|((p, n), c)| {
				return Vertex {
					pos: *p,
					normal: *n,
					color: *c,
					uv: vec2!(),
				};
			})
			.collect();

	}

	fn indices() -> &'static [u32] {
		return &[
			0, 1, 2,
			0, 2, 3,
			4, 5, 6,
			4, 6, 7,
			8, 9, 10,
			8, 10, 11,
			12, 13, 14,
			12, 14, 15,
			16, 17, 18,
			16, 18, 19,
			20, 21, 22,
			20, 22, 23,
		];
	}

}

