// wengwengweng

use std::ops;

use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mat4([f32; 16]);

impl Mat4 {

	pub fn new(m: [f32; 16]) -> Self {
		return Self(m);
	}

	pub fn identity() -> Self {
		return Self([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		]);
	}

	pub fn get(&self, x: usize, y: usize) -> Option<&f32> {
		return self.0.get(x * 4 + y);
	}

	pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut f32> {
		return self.0.get_mut(x * 4 + y);
	}

	pub fn translate(pos: Vec3) -> Self {
		return Self([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			pos.x, pos.y, pos.z, 1.0,
		]);
	}

	pub fn scale(scale: Vec3) -> Self {
		return Self([
			scale.x, 0.0, 0.0, 0.0,
			0.0, scale.y, 0.0, 0.0,
			0.0, 0.0, scale.z, 0.0,
			0.0, 0.0, 0.0, 1.0,
		]);
	}

	pub fn rotate(rot: f32, axis: Vec3) -> Self {

		let mut m = Self::identity();
		let c = rot.cos();
		let s = rot.sin();
		let cv = 1.0 - c;

		m.0[0] = (axis.x * axis.x * cv) + c;
		m.0[1] = (axis.x * axis.y * cv) + (axis.z * s);
		m.0[2] = (axis.x * axis.z * cv) - (axis.y * s);

		m.0[4] = (axis.y * axis.x * cv) - (axis.z * s);
		m.0[5] = (axis.y * axis.y * cv) + c;
		m.0[6] = (axis.y * axis.z * cv) + (axis.x * s);

		m.0[8] = (axis.z * axis.x * cv) + (axis.y * s);
		m.0[9] = (axis.z * axis.y * cv) - (axis.x * s);
		m.0[10] = (axis.z * axis.z * cv) + c;

		return m;

	}

	pub fn invert(&self) -> Self {

		let mut nm = [0.0; 16];
		let m = self.0;

		let f00 = m[10] * m[15] - m[14] * m[11];
		let f01 = m[9] * m[15] - m[13] * m[11];
		let f02 = m[9] * m[14] - m[13] * m[10];
		let f03 = m[8] * m[15] - m[12] * m[11];
		let f04 = m[8] * m[14] - m[12] * m[10];
		let f05 = m[8] * m[13] - m[12] * m[9];
		let f06 = m[6] * m[15] - m[14] * m[7];
		let f07 = m[5] * m[15] - m[13] * m[7];
		let f08 = m[5] * m[14] - m[13] * m[6];
		let f09 = m[4] * m[15] - m[12] * m[7];
		let f10 = m[4] * m[14] - m[12] * m[6];
		let f11 = m[5] * m[15] - m[13] * m[7];
		let f12 = m[4] * m[13] - m[12] * m[5];
		let f13 = m[6] * m[11] - m[10] * m[7];
		let f14 = m[5] * m[11] - m[9] * m[7];
		let f15 = m[5] * m[10] - m[9] * m[6];
		let f16 = m[4] * m[11] - m[8] * m[7];
		let f17 = m[4] * m[10] - m[8] * m[6];
		let f18 = m[4] * m[9] - m[8] * m[5];

		nm[0] = m[5] * f00 - m[6] * f01 + m[7] * f02;
		nm[4] = -(m[4] * f00 - m[6] * f03 + m[7] * f04);
		nm[8] = m[4] * f01 - m[5] * f03 + m[7] * f05;
		nm[12] = -(m[4] * f02 - m[5] * f04 + m[6] * f05);

		nm[1] = -(m[1] * f00 - m[2] * f01 + m[3] * f02);
		nm[5] = m[0] * f00 - m[2] * f03 + m[3] * f04;
		nm[9] = -(m[0] * f01 - m[1] * f03 + m[3] * f05);
		nm[13] = m[0] * f02 - m[1] * f04 + m[2] * f05;

		nm[2] = m[1] * f06 - m[2] * f07 + m[3] * f08;
		nm[6] = -(m[0] * f06 - m[2] * f09 + m[3] * f10);
		nm[10] = m[0] * f11 - m[1] * f09 + m[3] * f12;
		nm[14] = -(m[0] * f08 - m[1] * f10 + m[2] * f12);

		nm[3] = -(m[1] * f13 - m[2] * f14 + m[3] * f15);
		nm[7] = m[0] * f13 - m[2] * f16 + m[3] * f17;
		nm[11] = -(m[0] * f14 - m[1] * f16 + m[3] * f18);
		nm[15] = m[0] * f15 - m[1] * f17 + m[2] * f18;

		let det =
			m[0] * nm[0] +
			m[1] * nm[4] +
			m[2] * nm[8] +
			m[3] * nm[12];

		for i in 0..4 {
			for j in 0..4 {
				nm[i * 4 + j] *= (1.0 / det);
			}
		}

		return Self(nm);

	}

	pub fn as_arr(&self) -> [f32; 16] {
		return self.0;
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
				nm.0[i * 4 + j] =
					self.0[0 * 4 + j] * other.0[i * 4 + 0] +
					self.0[1 * 4 + j] * other.0[i * 4 + 1] +
					self.0[2 * 4 + j] * other.0[i * 4 + 2] +
					self.0[3 * 4 + j] * other.0[i * 4 + 3];
			}
		};

		return nm;

	}

}

impl ops::MulAssign for Mat4 {
	fn mul_assign(&mut self, pt: Self) {
		*self = *self * pt;
	}
}

impl ops::Mul<Vec4> for Mat4 {

	type Output = Vec4;

	fn mul(self, pt: Self::Output) -> Self::Output {

		let m = self.0;

		return vec4!(
			pt.x * m[0] + pt.y * m[4] + pt.z * m[8] + pt.w * m[12],
			pt.x * m[1] + pt.y * m[5] + pt.z * m[9] + pt.w * m[13],
			pt.x * m[2] + pt.y * m[6] + pt.z * m[10] + pt.w * m[14],
			pt.x * m[3] + pt.y * m[7] + pt.z * m[11] + pt.w * m[15]
		)

	}

}

impl ops::Mul<Vec3> for Mat4 {

	type Output = Vec3;

	fn mul(self, pt: Self::Output) -> Self::Output {
		let p = self * vec4!(pt.x, pt.y, pt.z, 1);
		return vec3!(p.x, p.y, p.z);
	}

}

impl ops::Mul<Vec2> for Mat4 {

	type Output = Vec2;

	fn mul(self, pt: Self::Output) -> Self::Output {
		let p = self * vec4!(pt.x, pt.y, 0, 1);
		return vec2!(p.x, p.y);
	}

}

