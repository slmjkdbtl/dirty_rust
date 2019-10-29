// wengwengweng

use std::mem;

use crate::math::*;
use super::gfx::*;

use crate::gl;

/// vertex layout for the 2d pipeline
#[derive(Clone)]
pub struct Vertex2D {
	pub pos: Vec3,
	pub uv: Vec2,
	pub color: Color,
}

impl gl::VertexLayout for Vertex2D {

	const STRIDE: usize = 9;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.pos.z,
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
			("a_pos", 3),
			("a_uv", 2),
			("a_color", 4),
		];
	}

}

/// vertex layout for the 3d pipeline
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

/// vertex layout for the cubemap pipeline
#[derive(Clone)]
pub struct VertexCubemap {
	pub pos: Vec3,
}

impl gl::VertexLayout for VertexCubemap {

	const STRIDE: usize = 3;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.pos.z,
		]);
	}

	fn attrs() -> gl::VertexAttrGroup {
		return &[
			("a_pos", 3),
		];
	}

}

/// uniform layout for the 2d pipeline
#[derive(Clone, PartialEq)]
pub(super) struct Uniform2D {
	pub proj: Mat4,
	pub tex: Texture,
	pub custom: Option<Vec<(&'static str, gl::UniformValue)>>,
}

impl gl::UniformLayout for Uniform2D {

	fn values(&self) -> UniformValues {

		let mut values: UniformValues = vec![
			("u_proj", &self.proj),
		];

		if let Some(custom) = &self.custom {
			for (name, v) in custom {
				values.push((name, v));
			}
		}

		return values;

	}

	fn texture(&self) -> Option<&dyn gl::Texture> {
		return Some(self.tex.gl_tex());
	}

}

/// uniform layout for the 3d pipeline
#[derive(Clone, PartialEq)]
pub(super) struct Uniform3D {
	pub proj: Mat4,
	pub view: Mat4,
	pub model: Transform,
	pub color: Color,
	pub tex: Texture,
	pub custom: Option<Vec<(&'static str, gl::UniformValue)>>,
}

impl gl::UniformLayout for Uniform3D {

	fn values(&self) -> UniformValues {

		let mut values: UniformValues = vec![
			("u_proj", &self.proj),
			("u_view", &self.view),
			("u_model", &self.model),
			("u_color", &self.color),
		];

		if let Some(custom) = &self.custom {
			for (name, v) in custom {
				values.push((name, v));
			}
		}

		return values;

	}

	fn texture(&self) -> Option<&dyn gl::Texture> {
		return Some(self.tex.gl_tex());
	}

}

/// uniform layout for the cubemap pipeline
#[derive(Clone, PartialEq)]
pub(super) struct UniformCubemap {
	pub proj: Mat4,
	pub view: Mat4,
	pub color: Color,
	pub tex: gl::CubemapTexture,
}

impl gl::UniformLayout for UniformCubemap {

	fn values(&self) -> UniformValues {
		return vec![
			("u_proj", &self.proj),
			("u_view", &self.view),
			("u_color", &self.color),
		];
	}

	fn texture(&self) -> Option<&dyn gl::Texture> {
		return Some(&self.tex);
	}

}

/// shape for a quad
pub(super) struct QuadShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub flip: Flip,
}

impl gl::Shape for QuadShape {

	type Vertex = Vertex2D;
	const COUNT: usize = 4;

	fn vertices(&self, queue: &mut Vec<f32>) {

		use gl::VertexLayout;

		let t = self.transform;
		let q = self.quad;
		let c = self.color;
		let p1 = t * (vec3!(-0.5, 0.5, 0.0));
		let p2 = t * (vec3!(0.5, 0.5, 0.0));
		let p3 = t * (vec3!(0.5, -0.5, 0.0));
		let p4 = t * (vec3!(-0.5, -0.5, 0.0));

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
/// shape for a flag
pub(super) struct Quad3DShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub flip: Flip,
}

impl gl::Shape for Quad3DShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 4;

	fn vertices(&self, queue: &mut Vec<f32>) {

		use gl::VertexLayout;

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

/// shape for a cube
pub(super) struct CubeShape;

impl gl::Shape for CubeShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 24;

	fn vertices(&self, queue: &mut Vec<f32>) {

		use gl::VertexLayout;

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
			color!(0, 0, 1, 1),
			color!(0, 1, 1, 1),
			color!(0, 1, 0, 1),
			color!(1, 1, 1, 1),
			color!(1, 1, 1, 1),
			color!(0, 1, 0, 1),
			color!(1, 1, 0, 1),
			color!(1, 0, 0, 1),
			color!(1, 0, 0, 1),
			color!(1, 1, 0, 1),
			color!(1, 1, 1, 1),
			color!(1, 0, 1, 1),
			color!(1, 0, 1, 1),
			color!(1, 1, 1, 1),
			color!(0, 1, 1, 1),
			color!(0, 0, 1, 1),
			color!(1, 1, 1, 1),
			color!(1, 0, 0, 1),
			color!(1, 0, 1, 1),
			color!(0, 0, 1, 1),
			color!(1, 1, 0, 1),
			color!(0, 1, 0, 1),
			color!(0, 1, 1, 1),
			color!(1, 1, 1, 1),
		];

		pos
			.iter()
			.zip(&normals)
			.zip(&colors)
			// zoop
			.for_each(|((p, n), c)| {
				Vertex3D {
					pos: *p,
					normal: *n,
					color: *c,
					uv: vec2!(),
				}.push(queue);
			});

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

/// shape for a cubemap
pub(super) struct CubemapShape;

impl gl::Shape for CubemapShape {

	type Vertex = VertexCubemap;
	const COUNT: usize = 8;

	fn vertices(&self, queue: &mut Vec<f32>) {

		use gl::VertexLayout;

		let pos = [
			vec3!(-1, -1, 1),
			vec3!(-1, 1, 1),
			vec3!(1, 1, 1),
			vec3!(1, -1, 1),
			vec3!(-1, -1, -1),
			vec3!(-1, 1, -1),
			vec3!(1, 1, -1),
			vec3!(1, -1, -1),
		];

		pos
			.into_iter()
			.for_each(|p| {
				VertexCubemap {
					pos: *p,
				}.push(queue);
			});

	}

	fn indices() -> &'static [u32] {
		return &[
			0, 2, 1, 0, 3, 2,
			4, 3, 0, 4, 7, 3,
			4, 1, 5, 4, 0, 1,
			3, 6, 2, 3, 7, 6,
			1, 6, 5, 1, 2, 6,
			7, 5, 6, 7, 4, 5,
		];
	}

}

