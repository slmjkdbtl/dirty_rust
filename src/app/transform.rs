// wengwengweng

use std::ops;

use crate::*;
use crate::math::*;

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

impl ops::Mul<Vec4> for Transform {
	type Output = Vec4;
	fn mul(self, pt: Self::Output) -> Self::Output {
		return self.matrix * pt;
	}
}

impl ops::Mul<Vec3> for Transform {
	type Output = Vec3;
	fn mul(self, pt: Self::Output) -> Self::Output {
		return self.matrix * pt;
	}
}

impl ops::Mul<Vec2> for Transform {
	type Output = Vec2;
	fn mul(self, pt: Self::Output) -> Self::Output {
		return self.matrix * pt;
	}
}

pub fn t() -> Transform {
	return Transform::new();
}

