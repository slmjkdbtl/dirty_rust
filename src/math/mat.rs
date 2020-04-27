// wengwengweng

use std::ops;
use serde::Serialize;
use serde::Deserialize;

use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mat4 {
	m: [f32; 16],
}

impl Mat4 {

	pub fn new(m: [f32; 16]) -> Self {
		return Self {
			m: m,
		};
	}

	pub fn identity() -> Self {
		return Self::new([
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		]);
	}

	pub fn get(&self, x: usize, y: usize) -> Option<&f32> {
		return self.m.get(x * 4 + y);
	}

	pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut f32> {
		return self.m.get_mut(x * 4 + y);
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

	// TODO: 3d
	pub fn skew(sk: Vec3) -> Self {
		return Self::new([
			1.0, sk.y, 0.0, 0.0,
			sk.x, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		]);
	}

	pub fn rotate(rot: f32, a: Vec3) -> Self {

		let mut m = Self::identity();
		let c = rot.cos();
		let s = rot.sin();
		let cv = 1.0 - c;

		m.m[0] = (a.x * a.x * cv) + c;
		m.m[1] = (a.x * a.y * cv) + (a.z * s);
		m.m[2] = (a.x * a.z * cv) - (a.y * s);

		m.m[4] = (a.y * a.x * cv) - (a.z * s);
		m.m[5] = (a.y * a.y * cv) + c;
		m.m[6] = (a.y * a.z * cv) + (a.x * s);

		m.m[8] = (a.z * a.x * cv) + (a.y * s);
		m.m[9] = (a.z * a.y * cv) - (a.x * s);
		m.m[10] = (a.z * a.z * cv) + c;

		return m;

	}

	pub fn rotate_quat(q: Vec4) -> Self {

		return Self::new([
			q.w, q.z, -q.y, q.x,
			-q.z, q.w, q.x, q.y,
			q.y, -q.x, q.w, q.z,
			-q.x, -q.y, -q.z, q.w,
		]) * Self::new([
			q.w, q.z, -q.y, -q.x,
			-q.z, q.w, q.x, -q.y,
			q.y, -q.x, q.w, -q.z,
			q.x, q.y, q.z, q.w,
		]);

	}

	pub fn inverse(&self) -> Self {

		let mut out = [0.0; 16];
		let m = self.m;

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

		out[0] = m[5] * f00 - m[6] * f01 + m[7] * f02;
		out[4] = -(m[4] * f00 - m[6] * f03 + m[7] * f04);
		out[8] = m[4] * f01 - m[5] * f03 + m[7] * f05;
		out[12] = -(m[4] * f02 - m[5] * f04 + m[6] * f05);

		out[1] = -(m[1] * f00 - m[2] * f01 + m[3] * f02);
		out[5] = m[0] * f00 - m[2] * f03 + m[3] * f04;
		out[9] = -(m[0] * f01 - m[1] * f03 + m[3] * f05);
		out[13] = m[0] * f02 - m[1] * f04 + m[2] * f05;

		out[2] = m[1] * f06 - m[2] * f07 + m[3] * f08;
		out[6] = -(m[0] * f06 - m[2] * f09 + m[3] * f10);
		out[10] = m[0] * f11 - m[1] * f09 + m[3] * f12;
		out[14] = -(m[0] * f08 - m[1] * f10 + m[2] * f12);

		out[3] = -(m[1] * f13 - m[2] * f14 + m[3] * f15);
		out[7] = m[0] * f13 - m[2] * f16 + m[3] * f17;
		out[11] = -(m[0] * f14 - m[1] * f16 + m[3] * f18);
		out[15] = m[0] * f15 - m[1] * f17 + m[2] * f18;

		let det =
			m[0] * out[0] +
			m[1] * out[4] +
			m[2] * out[8] +
			m[3] * out[12];

		for i in 0..4 {
			for j in 0..4 {
				out[i * 4 + j] *= (1.0 / det);
			}
		}

// 		out[0] =
// 			m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15] + m[9] * m[7] * m[14] + m[13] * m[6] * m[11] - m[13] * m[7] * m[10];
// 		out[1] =
// 			-m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15] - m[9] * m[3] * m[14] - m[13] * m[2] * m[11] + m[13] * m[3] * m[10];
// 		out[2] =
// 			m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15] + m[5] * m[3] * m[14] + m[13] * m[2] * m[7] - m[13] * m[3] * m[6];
// 		out[3] =
// 			-m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11] - m[5] * m[3] * m[10] - m[9] * m[2] * m[7] + m[9] * m[3] * m[6];
// 		out[4] =
// 			-m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15] - m[8] * m[7] * m[14] - m[12] * m[6] * m[11] + m[12] * m[7] * m[10];
// 		out[5] =
// 			m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15] + m[8] * m[3] * m[14] + m[12] * m[2] * m[11] - m[12] * m[3] * m[10];
// 		out[6] =
// 			-m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15] - m[4] * m[3] * m[14] - m[12] * m[2] * m[7] + m[12] * m[3] * m[6];
// 		out[7] =
// 			m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11] + m[4] * m[3] * m[10] + m[8] * m[2] * m[7] - m[8] * m[3] * m[6];
// 		out[8] =
// 			m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15] + m[8] * m[7] * m[13] + m[12] * m[5] * m[11] - m[12] * m[7] * m[9];
// 		out[9] =
// 			-m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15] - m[8] * m[3] * m[13] - m[12] * m[1] * m[11] + m[12] * m[3] * m[9];
// 		out[10] =
// 			m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15] + m[4] * m[3] * m[13] + m[12] * m[1] * m[7] - m[12] * m[3] * m[5];
// 		out[11] =
// 			-m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11] - m[4] * m[3] * m[9] - m[8] * m[1] * m[7] + m[8] * m[3] * m[5];
// 		out[12] =
// 			-m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14] - m[8] * m[6] * m[13] - m[12] * m[5] * m[10] + m[12] * m[6] * m[9];
// 		out[13] =
// 			m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14] + m[8] * m[2] * m[13] + m[12] * m[1] * m[10] - m[12] * m[2] * m[9];
// 		out[14] =
// 			-m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14] - m[4] * m[2] * m[13] - m[12] * m[1] * m[6] + m[12] * m[2] * m[5];
// 		out[15] =
// 			m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10] + m[4] * m[2] * m[9] + m[8] * m[1] * m[6] - m[8] * m[2] * m[5];

		return Self::new(out);

	}

	pub fn as_arr(&self) -> [f32; 16] {
		return self.m;
	}

	pub fn remove_translation(&self) -> Self {

		let mut out = self.clone();

		if let Some(val) = out.get_mut(3, 0) {
			*val = 0.0;
		}

		if let Some(val) = out.get_mut(3, 1) {
			*val = 0.0;
		}

		if let Some(val) = out.get_mut(3, 2) {
			*val = 0.0;
		}

		return out;

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

		let mut out = Self::identity();

		for i in 0..4 {
			for j in 0..4 {
				out.m[i * 4 + j] =
					self.m[0 * 4 + j] * other.m[i * 4 + 0] +
					self.m[1 * 4 + j] * other.m[i * 4 + 1] +
					self.m[2 * 4 + j] * other.m[i * 4 + 2] +
					self.m[3 * 4 + j] * other.m[i * 4 + 3];
			}
		};

		return out;

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

		let m = self.m;

		return vec4!(
			pt.x * m[0] + pt.y * m[4] + pt.z * m[8] + pt.w * m[12],
			pt.x * m[1] + pt.y * m[5] + pt.z * m[9] + pt.w * m[13],
			pt.x * m[2] + pt.y * m[6] + pt.z * m[10] + pt.w * m[14],
			pt.x * m[3] + pt.y * m[7] + pt.z * m[11] + pt.w * m[15]
		);

	}

}

impl ops::Mul<Vec3> for Mat4 {

	type Output = Vec4;

	fn mul(self, pt: Vec3) -> Self::Output {
		return self * vec4!(pt.x, pt.y, pt.z, 1);
	}

}

impl ops::Mul<Vec2> for Mat4 {

	type Output = Vec4;

	fn mul(self, pt: Vec2) -> Self::Output {
		return self * vec4!(pt.x, pt.y, 0, 1);
	}

}

#[macro_export]
macro_rules! mat4 {
	() => {
		$crate::math::Mat4::identity()
	};
	($($v:expr),+$(,)?) => {
		$crate::math::Mat4::new([ $($v,)+ ])
	};
}

pub use mat4;

