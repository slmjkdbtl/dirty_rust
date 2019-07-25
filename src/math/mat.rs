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
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			pos.x, pos.y, pos.z, 1.0,
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

	pub fn forward(&self, pt: Vec4) -> Vec4 {

		let m = self.m;

		return vec4!(
			pt.x * m[0] + pt.y * m[4] + pt.z * m[8] + pt.w * m[12],
			pt.x * m[1] + pt.y * m[5] + pt.z * m[9] + pt.w * m[13],
			pt.x * m[2] + pt.y * m[6] + pt.z * m[10] + pt.w * m[14],
			pt.x * m[3] + pt.y * m[7] + pt.z * m[11] + pt.w * m[15]
		)

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
