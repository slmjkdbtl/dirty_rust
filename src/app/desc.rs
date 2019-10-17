// wengwengweng

use std::mem;

use crate::math::*;
use super::gfx::*;

use crate::gl;

#[derive(Clone)]
pub struct Vertex2D {
	pub pos: Vec2,
	pub uv: Vec2,
	pub color: Color,
}

impl gl::VertexLayout for Vertex2D {

	const STRIDE: usize = 8;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.uv.x,
			self.uv.y,
			self.color.r,
			self.color.g,
			self.color.b,
			self.color.a,
		]);
	}

	fn attrs() -> gl::VertexAttrGroup {
		return &[
			("a_pos", 2),
			("a_uv", 2),
			("a_color", 4),
		];
	}

}

#[derive(Clone)]
pub struct Vertex3D {
	pub pos: Vec3,
	pub uv: Vec2,
	pub normal: Vec3,
	pub color: Color,
}

impl gl::VertexLayout for Vertex3D {

	const STRIDE: usize = 12;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.pos.z,
			self.uv.x,
			self.uv.y,
			self.normal.x,
			self.normal.y,
			self.normal.z,
			self.color.r,
			self.color.g,
			self.color.b,
			self.color.a,
		]);
	}

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
pub(super) struct Uniform2D {
	pub proj: Mat4,
	pub tex: Texture,
	pub custom: Option<UniformValues>,
}

impl gl::UniformLayout for Uniform2D {

	fn values(&self) -> UniformValues {

		let mut values = vec![
			("u_proj", self.proj.into()),
		];

		if let Some(custom) = &self.custom {
			values.extend(custom.clone());
		}

		return values;

	}

	fn texture(&self) -> Option<&gl::Texture> {
		return Some(&self.tex.handle);
	}

}

#[derive(Clone, PartialEq)]
pub(super) struct Uniform3D {

	pub proj: Mat4,
	pub view: Mat4,
	pub model: Transform,
	pub color: Color,
	pub tex: Texture,
	pub custom: Option<UniformValues>,

}

impl gl::UniformLayout for Uniform3D {

	fn values(&self) -> UniformValues {

		let mut values = vec![
			("u_proj", self.proj.into()),
			("u_view", self.view.into()),
			("u_model", self.model.as_mat4().into()),
			("u_color", self.color.into()),
		];

		if let Some(custom) = &self.custom {
			values.extend(custom.clone());
		}

		return values;

	}

	fn texture(&self) -> Option<&gl::Texture> {
		return Some(&self.tex.handle);
	}

}

pub(super) struct QuadShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub flip: Flip,
}

impl Shape for QuadShape {

	type Vertex = Vertex2D;
	const COUNT: usize = 4;

	fn vertices(&self, queue: &mut Vec<f32>) {

		let t = self.transform;
		let q = self.quad;
		let c = self.color;
		let p1 = t * (vec2!(-0.5, 0.5));
		let p2 = t * (vec2!(0.5, 0.5));
		let p3 = t * (vec2!(0.5, -0.5));
		let p4 = t * (vec2!(-0.5, -0.5));

		let mut u1 = vec2!(q.x, q.y + q.h);
		let mut u2 = vec2!(q.x + q.w, q.y + q.h);
		let mut u3 = vec2!(q.x + q.w, q.y);
		let mut u4 = vec2!(q.x, q.y);

		match self.flip {
			Flip::X => {
				mem::swap(&mut u1, &mut u2);
				mem::swap(&mut u4, &mut u3);
			},
			Flip::Y => {
				mem::swap(&mut u2, &mut u3);
				mem::swap(&mut u1, &mut u4);
			},
			Flip::XY => {
				mem::swap(&mut u2, &mut u4);
				mem::swap(&mut u1, &mut u3);
			},
			_ => {},
		}

		Vertex2D {
			pos: p1,
			uv: u1,
			color: c
		}.push(queue);

		Vertex2D {
			pos: p2,
			uv: u2,
			color: c
		}.push(queue);

		Vertex2D {
			pos: p3,
			uv: u3,
			color: c
		}.push(queue);

		Vertex2D {
			pos: p4,
			uv: u4,
			color: c
		}.push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[0, 1, 3, 1, 2, 3];
	}

}

// TODO: messy
pub(super) struct FlagShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub flip: Flip,
}

impl Shape for FlagShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 4;

	fn vertices(&self, queue: &mut Vec<f32>) {

		let t = self.transform;
		let q = self.quad;
		let c = self.color;
		let p1 = t * (vec2!(-0.5, 0.5));
		let p2 = t * (vec2!(0.5, 0.5));
		let p3 = t * (vec2!(0.5, -0.5));
		let p4 = t * (vec2!(-0.5, -0.5));

		let mut u1 = vec2!(q.x, q.y + q.h);
		let mut u2 = vec2!(q.x + q.w, q.y + q.h);
		let mut u3 = vec2!(q.x + q.w, q.y);
		let mut u4 = vec2!(q.x, q.y);

		match self.flip {
			Flip::X => {
				mem::swap(&mut u1, &mut u2);
				mem::swap(&mut u4, &mut u3);
			},
			Flip::Y => {
				mem::swap(&mut u2, &mut u3);
				mem::swap(&mut u1, &mut u4);
			},
			Flip::XY => {
				mem::swap(&mut u2, &mut u4);
				mem::swap(&mut u1, &mut u3);
			},
			_ => {},
		}

		Vertex3D {
			pos: vec3!(p1.x, p1.y, 0.0),
			uv: u1,
			normal: vec3!(0, 0, -1),
			color: c,
		}.push(queue);

		Vertex3D {
			pos: vec3!(p2.x, p2.y, 0.0),
			uv: u2,
			normal: vec3!(0, 0, -1),
			color: c,
		}.push(queue);

		Vertex3D {
			pos: vec3!(p3.x, p3.y, 0.0),
			uv: u3,
			normal: vec3!(0, 0, -1),
			color: c,
		}.push(queue);

		Vertex3D {
			pos: vec3!(p4.x, p4.y, 0.0),
			uv: u4,
			normal: vec3!(0, 0, -1),
			color: c,
		}.push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[0, 1, 3, 1, 2, 3];
	}

}

pub(super) struct CubeShape;

impl Shape for CubeShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 8;

	fn vertices(&self, queue: &mut Vec<f32>) {

		Vertex3D {
			pos: vec3!(-0.5, -0.5, 0.5),
			uv: vec2!(),
			normal: vec3!(-0.41, -0.41, 0.82),
			color: color!(1, 0, 0, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(0.5, -0.5, 0.5),
			uv: vec2!(),
			normal: vec3!(0.67, -0.67, 0.33),
			color: color!(0, 1, 0, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(0.5, 0.5, 0.5),
			uv: vec2!(),
			normal: vec3!(0.41, 0.41, 0.82),
			color: color!(0, 0, 1, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(-0.5, 0.5, 0.5),
			uv: vec2!(),
			normal: vec3!(-0.67, 0.67, 0.33),
			color: color!(1, 1, 1, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(-0.5, -0.5, -0.5),
			uv: vec2!(),
			normal: vec3!(-0.67, -0.67, -0.33),
			color: color!(1, 0, 0, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(0.5, -0.5, -0.5),
			uv: vec2!(),
			normal: vec3!(0.41, -0.41, -0.82),
			color: color!(0, 1, 0, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(0.5, 0.5, -0.5),
			uv: vec2!(),
			normal: vec3!(0.67, 0.67, -0.33),
			color: color!(0, 0, 1, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(-0.5, 0.5, -0.5),
			uv: vec2!(),
			normal: vec3!(-0.41, 0.41, -0.82),
			color: color!(1, 1, 1, 1),
		}.push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[
			0, 1, 2,
			2, 3, 0,
			1, 5, 6,
			6, 2, 1,
			7, 6, 5,
			5, 4, 7,
			4, 0, 3,
			3, 7, 4,
			4, 5, 1,
			1, 0, 4,
			3, 2, 6,
			6, 7, 3,
		];
	}

}

