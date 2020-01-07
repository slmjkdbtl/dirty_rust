// wengwengweng

use std::ops;

use crate::*;
use crate::math::*;

/// transform matrix
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Transform {
	matrix: Mat4,
}

impl Transform {

	pub fn new() -> Self {
		return Self::from_mat4(Mat4::identity());
	}

	pub fn from_mat4(m: Mat4) -> Self {
		return Self {
			matrix: m,
		};
	}

	pub fn translate(&self, p: Vec3) -> Self {
		return Self::from_mat4(self.matrix * Mat4::translate(p));
	}

	pub fn scale(&self, s: Vec3) -> Self {
		return Self::from_mat4(self.matrix * Mat4::scale(s));
	}

	pub fn rotate(&self, angle: f32, axis: Vec3) -> Self {
		return Self::from_mat4(self.matrix *  Mat4::rotate(angle, axis));
	}

	pub fn rotate_quat(&self, q: Vec4) -> Self {
		return Self::from_mat4(self.matrix *  Mat4::rotate_quat(q));
	}

	pub fn skew(&self, s: Vec3) -> Self {
		return Self::from_mat4(self.matrix * Mat4::skew(s));
	}

	pub fn as_mat4(&self) -> Mat4 {
		return self.matrix;
	}

	pub fn inverse(&self) -> Self {
		return Self::from_mat4(self.matrix.inverse());
	}

	pub fn apply(self, other: &Self) -> Self {
		return Self::from_mat4(self.matrix * other.matrix);
	}

}

// aliases
impl Transform {

	pub fn t3(&self, p: Vec3) -> Self {
		return self.translate(p);
	}

	pub fn t2(&self, p: Vec2) -> Self {
		return self.t3(vec3!(p.x, p.y, 0.0));
	}

	pub fn tx(&self, dx: f32) -> Self {
		return self.t3(vec3!(dx, 0, 0));
	}

	pub fn ty(&self, dy: f32) -> Self {
		return self.t3(vec3!(0, dy, 0));
	}

	pub fn tz(&self, dz: f32) -> Self {
		return self.t3(vec3!(0, 0, dz));
	}

	pub fn r(&self, a: f32) -> Self {
		return self.rz(a);
	}

	pub fn rx(&self, a: f32) -> Self {
		return self.rotate(a, vec3!(1, 0, 0));
	}

	pub fn ry(&self, a: f32) -> Self {
		return self.rotate(a, vec3!(0, 1, 0));
	}

	pub fn rz(&self, a: f32) -> Self {
		return self.rotate(a, vec3!(0, 0, 1));
	}

	pub fn rq(&self, q: Vec4) -> Self {
		return self.rotate_quat(q);
	}

	pub fn s3(&self, s: Vec3) -> Self {
		return self.scale(s);
	}

	pub fn s2(&self, s: Vec2) -> Self {
		return self.s3(vec3!(s.x, s.y, 1.0));
	}

	pub fn sx(&self, x: f32) -> Self {
		return self.s3(vec3!(x, 1, 1));
	}

	pub fn sy(&self, y: f32) -> Self {
		return self.s3(vec3!(1, y, 1));
	}

	pub fn sz(&self, z: f32) -> Self {
		return self.s3(vec3!(1, 1, z));
	}

	pub fn sk2(&self, s: Vec2) -> Self {
		return self.skew(vec3!(s.x, s.y, 0));
	}

	pub fn skx(&self, x: f32) -> Self {
		return self.sk2(vec2!(x, 0));
	}

	pub fn sky(&self, y: f32) -> Self {
		return self.sk2(vec2!(0, y));
	}

}

impl gl::IntoUniformValue for Transform {
	fn into_uniform(&self) -> gl::UniformValue {
		return gl::UniformValue::Mat4(self.as_mat4().as_arr());
	}
}

impl ops::Mul<Transform> for Mat4 {
	type Output = Mat4;
	fn mul(self, m: Transform) -> Self::Output {
		return self * m.as_mat4();
	}
}

impl ops::Mul<Vec4> for Transform {
	type Output = Vec4;
	fn mul(self, pt: Vec4) -> Self::Output {
		return self.matrix * pt;
	}
}

impl ops::Mul<Vec3> for Transform {
	type Output = Vec3;
	fn mul(self, pt: Vec3) -> Self::Output {
		return self.matrix * pt;
	}
}

impl ops::Mul<Vec2> for Transform {
	type Output = Vec2;
	fn mul(self, pt: Vec2) -> Self::Output {
		return self.matrix * pt;
	}
}

pub fn t() -> Transform {
	return Transform::new();
}

