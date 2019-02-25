// wengwengweng

use std::ops;

use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dir {
	X,
	Y,
	Z,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4 {
	pub(super) m: [[f32; 4]; 4],
}

impl Mat4 {

	pub fn new(m: [[f32; 4]; 4]) -> Self {
		return Self {
			m: m,
		};
	}

	pub fn identity() -> Self {

		return Self {
			m: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0],
			]
		};

	}

	pub fn translate(self, pos: Vec3) -> Self {

		let mut m = Self::identity();

		m.m[3][0] = pos.x;
		m.m[3][1] = pos.y;
		m.m[3][2] = pos.z;

		return self * m;

	}

	pub fn scale(self, scale: Vec3) -> Self {

		let mut m = Self::identity();

		m.m[0][0] = scale.x;
		m.m[1][1] = scale.y;
		m.m[2][2] = scale.z;

		return self * m;

	}

	pub fn rotate(self, rot: f32, dir: Dir) -> Self {

		let mut m = Self::identity();
		let c = rot.cos();
		let s = rot.sin();
		let cv = 1.0 - c;

		let axis = match dir {
			Dir::X => vec3!(1, 0, 0),
			Dir::Y => vec3!(0, 1, 0),
			Dir::Z => vec3!(0, 0, 1),
		};

		m.m[0][0] = (axis.x * axis.x * cv) + c;
		m.m[0][1] = (axis.x * axis.y * cv) + (axis.z * s);
		m.m[0][2] = (axis.x * axis.z * cv) - (axis.y * s);

		m.m[1][0] = (axis.y * axis.x * cv) - (axis.z * s);
		m.m[1][1] = (axis.y * axis.y * cv) + c;
		m.m[1][2] = (axis.y * axis.z * cv) + (axis.x * s);

		m.m[2][0] = (axis.z * axis.x * cv) + (axis.y * s);
		m.m[2][1] = (axis.z * axis.y * cv) - (axis.x * s);
		m.m[2][2] = (axis.z * axis.z * cv) + c;

		return self * m;

	}

	pub fn inverse(&self) -> Self {

		let mut nm = Self::identity();

		let f00 = self.m[2][2] * self.m[3][3] - self.m[3][2] * self.m[2][3];
		let f01 = self.m[2][1] * self.m[3][3] - self.m[3][1] * self.m[2][3];
		let f02 = self.m[2][1] * self.m[3][2] - self.m[3][1] * self.m[2][2];
		let f03 = self.m[2][0] * self.m[3][3] - self.m[3][0] * self.m[2][3];
		let f04 = self.m[2][0] * self.m[3][2] - self.m[3][0] * self.m[2][2];
		let f05 = self.m[2][0] * self.m[3][1] - self.m[3][0] * self.m[2][1];
		let f06 = self.m[1][2] * self.m[3][3] - self.m[3][2] * self.m[1][3];
		let f07 = self.m[1][1] * self.m[3][3] - self.m[3][1] * self.m[1][3];
		let f08 = self.m[1][1] * self.m[3][2] - self.m[3][1] * self.m[1][2];
		let f09 = self.m[1][0] * self.m[3][3] - self.m[3][0] * self.m[1][3];
		let f10 = self.m[1][0] * self.m[3][2] - self.m[3][0] * self.m[1][2];
		let f11 = self.m[1][1] * self.m[3][3] - self.m[3][1] * self.m[1][3];
		let f12 = self.m[1][0] * self.m[3][1] - self.m[3][0] * self.m[1][1];
		let f13 = self.m[1][2] * self.m[2][3] - self.m[2][2] * self.m[1][3];
		let f14 = self.m[1][1] * self.m[2][3] - self.m[2][1] * self.m[1][3];
		let f15 = self.m[1][1] * self.m[2][2] - self.m[2][1] * self.m[1][2];
		let f16 = self.m[1][0] * self.m[2][3] - self.m[2][0] * self.m[1][3];
		let f17 = self.m[1][0] * self.m[2][2] - self.m[2][0] * self.m[1][2];
		let f18 = self.m[1][0] * self.m[2][1] - self.m[2][0] * self.m[1][1];

		nm.m[0][0] = self.m[1][1] * f00 - self.m[1][2] * f01 + self.m[1][3] * f02;
		nm.m[1][0] = -(self.m[1][0] * f00 - self.m[1][2] * f03 + self.m[1][3] * f04);
		nm.m[2][0] = self.m[1][0] * f01 - self.m[1][1] * f03 + self.m[1][3] * f05;
		nm.m[3][0] = -(self.m[1][0] * f02 - self.m[1][1] * f04 + self.m[1][2] * f05);

		nm.m[0][1] = -(self.m[0][1] * f00 - self.m[0][2] * f01 + self.m[0][3] * f02);
		nm.m[1][1] = self.m[0][0] * f00 - self.m[0][2] * f03 + self.m[0][3] * f04;
		nm.m[2][1] = -(self.m[0][0] * f01 - self.m[0][1] * f03 + self.m[0][3] * f05);
		nm.m[3][1] = self.m[0][0] * f02 - self.m[0][1] * f04 + self.m[0][2] * f05;

		nm.m[0][2] = self.m[0][1] * f06 - self.m[0][2] * f07 + self.m[0][3] * f08;
		nm.m[1][2] = -(self.m[0][0] * f06 - self.m[0][2] * f09 + self.m[0][3] * f10);
		nm.m[2][2] = self.m[0][0] * f11 - self.m[0][1] * f09 + self.m[0][3] * f12;
		nm.m[3][2] = -(self.m[0][0] * f08 - self.m[0][1] * f10 + self.m[0][2] * f12);

		nm.m[0][3] = -(self.m[0][1] * f13 - self.m[0][2] * f14 + self.m[0][3] * f15);
		nm.m[1][3] = self.m[0][0] * f13 - self.m[0][2] * f16 + self.m[0][3] * f17;
		nm.m[2][3] = -(self.m[0][0] * f14 - self.m[0][1] * f16 + self.m[0][3] * f18);
		nm.m[3][3] = self.m[0][0] * f15 - self.m[0][1] * f17 + self.m[0][2] * f18;

		let det =
			self.m[0][0] * nm.m[0][0] +
			self.m[0][1] * nm.m[1][0] +
			self.m[0][2] * nm.m[2][0] +
			self.m[0][3] * nm.m[3][0];

		for i in 0..4 {
			for j in 0..4 {
				nm.m[i][j] *= (1.0 / det);
			}
		}

		return nm;

	}

	pub fn forward(&self, pt: Vec2) -> Vec2 {

		let mut npt = vec2!();

		npt.x = pt.x * self.m[0][0] + pt.y * self.m[1][0] + self.m[3][0];
		npt.y = pt.x * self.m[0][1] + pt.y * self.m[1][1] + self.m[3][1];

		return npt;

	}

	pub fn as_arr(&self) -> [[f32; 4]; 4] {
		return self.m;
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

		let mut nm = Mat4::identity();

		for i in 0..4 {
			for j in 0..4 {
				nm.m[i][j] =
					self.m[0][j] * other.m[i][0] +
					self.m[1][j] * other.m[i][1] +
					self.m[2][j] * other.m[i][2] +
					self.m[3][j] * other.m[i][3];
			}
		};

		return nm;

	}

}
