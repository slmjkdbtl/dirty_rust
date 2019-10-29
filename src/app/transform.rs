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

	pub fn translate(&self, p: Vec2) -> Self {
		return Self::from_mat4(self.matrix * Mat4::translate(vec3!(p.x, p.y, 0.0)));
	}

	pub fn rotate(&self, a: f32) -> Self {
		return Self::from_mat4(self.matrix * Mat4::rotate(a, vec3!(0, 0, 1)));
	}

	pub fn scale(&self, s: Vec2) -> Self {
		return Self::from_mat4(self.matrix * Mat4::scale(vec3!(s.x, s.y, 1.0)));
	}

	pub fn translate_3d(&self, p: Vec3) -> Self {
		return Self::from_mat4(self.matrix * Mat4::translate(p));
	}

	pub fn scale_3d(&self, s: Vec3) -> Self {
		return Self::from_mat4(self.matrix * Mat4::scale(s));
	}

	pub fn rotate_x(&self, a: f32) -> Self {
		return Self::from_mat4(self.matrix *  Mat4::rotate(a, vec3!(1, 0, 0)));
	}

	pub fn rotate_y(&self, a: f32) -> Self {
		return Self::from_mat4(self.matrix *  Mat4::rotate(a, vec3!(0, 1, 0)));
	}

	pub fn rotate_z(&self, a: f32) -> Self {
		return Self::from_mat4(self.matrix *  Mat4::rotate(a, vec3!(0, 0, 1)));
	}

	pub fn as_mat4(&self) -> Mat4 {
		return self.matrix;
	}

	pub fn invert(&self) -> Self {
		return Self::from_mat4(self.matrix.invert());
	}

	pub fn apply(self, other: &Self) -> Self {
		return Self::from_mat4(self.matrix * other.matrix);
	}

}

// aliases
impl Transform {

	pub fn t(&self, p: Vec2) -> Self {
		return self.translate(p);
	}

	pub fn r(&self, a: f32) -> Self {
		return self.rotate(a);
	}

	pub fn s(&self, s: Vec2) -> Self {
		return self.scale(s);
	}

	pub fn t3(&self, p: Vec3) -> Self {
		return self.translate_3d(p);
	}

	pub fn rx(&self, a: f32) -> Self {
		return self.rotate_x(a);
	}

	pub fn ry(&self, a: f32) -> Self {
		return self.rotate_y(a);
	}

	pub fn rz(&self, a: f32) -> Self {
		return self.rotate_z(a);
	}

	pub fn s3(&self, s: Vec3) -> Self {
		return self.scale_3d(s);
	}

}

impl gl::IntoUniformValue for Transform {
	fn into(&self) -> gl::UniformValue {
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

