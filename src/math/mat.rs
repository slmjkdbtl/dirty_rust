// wengwengweng

use std::ops;

use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dir {
	X,
	Y,
	Z,
}

impl From<Dir> for Vec3 {
	fn from(d: Dir) -> Vec3 {
		return match d {
			Dir::X => vec3!(1, 0, 0),
			Dir::Y => vec3!(0, 1, 0),
			Dir::Z => vec3!(0, 0, 1),
		};
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
	pub(super) m: [f32; 16],
}

impl Mat4 {

	pub fn new(m: [f32; 16]) -> Self {
		return Self {
			m: m,
		};
	}

	pub fn identity() -> Self {

		return Self {
			m: [
				1.0, 0.0, 0.0, 0.0,
				0.0, 1.0, 0.0, 0.0,
				0.0, 0.0, 1.0, 0.0,
				0.0, 0.0, 0.0, 1.0,
			]
		};

	}

	pub fn translate(pos: Vec3) -> Self {
		return Self::new([
			1.0, 0.0, 0.0, pos.x,
			0.0, 1.0, 0.0, pos.y,
			0.0, 0.0, 1.0, pos.z,
			0.0, 0.0, 0.0, 1.0,
		]);
	}

	pub fn scale(scale: Vec3) -> Self {
		return Self::new([
			scale.x, 0.0, 0.0, 0.0,
			0.0, scale.y, 0.0, 0.0,
			0.0, 0.0, scale.z, 0.0,
			0.0, 0.0, 0.0, 1.0,
		]);
	}

	pub fn rotate(rot: f32, dir: Dir) -> Self {

		let mut m = Self::identity();
		let c = rot.cos();
		let s = rot.sin();
		let cv = 1.0 - c;
		let axis: Vec3 = dir.into();

		m.m[0] = (axis.x * axis.x * cv) + c;
		m.m[1] = (axis.x * axis.y * cv) + (axis.z * s);
		m.m[2] = (axis.x * axis.z * cv) - (axis.y * s);

		m.m[4] = (axis.y * axis.x * cv) - (axis.z * s);
		m.m[5] = (axis.y * axis.y * cv) + c;
		m.m[6] = (axis.y * axis.z * cv) + (axis.x * s);

		m.m[8] = (axis.z * axis.x * cv) + (axis.y * s);
		m.m[9] = (axis.z * axis.y * cv) - (axis.x * s);
		m.m[10] = (axis.z * axis.z * cv) + c;

		return m;

	}

	pub fn as_arr(&self) -> [f32; 16] {
		return self.m;
	}

	pub fn forward(&self, v: Vec4) -> Vec4 {

		let m = self.m;

		return vec4!(
			m[0] * v.x + m[1] * v.y + m[2] * v.z + m[3] * v.w,
			m[4] * v.x + m[5] * v.y + m[6] * v.z + m[7] * v.w,
			m[8] * v.x + m[9] * v.y + m[10] * v.z + m[11] * v.w,
			m[12] * v.x + m[13] * v.y + m[14] * v.z + m[15] * v.w
		);

	}

}

impl Default for Mat4 {

	fn default() -> Self {
		return Self::identity();
	}

}

impl ops::Mul for Mat4 {

	type Output = Self;

	fn mul(self, other: Self) -> Self {

		let mut nm = Self::identity();

		for i in 0..4 {
			for j in 0..4 {
				nm.m[i * 4 + j] =
					self.m[0 * 4 + j] * other.m[i * 4 + 0] +
					self.m[1 * 4 + j] * other.m[i * 4 + 1] +
					self.m[2 * 4 + j] * other.m[i * 4 + 2] +
					self.m[3 * 4 + j] * other.m[i * 4 + 3];
			}
		};

		return nm;

	}

}
