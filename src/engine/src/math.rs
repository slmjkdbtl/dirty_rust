// wengwengweng

use core::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
	m: [[f32; 4]; 4],
}

impl Vector2 {
	pub fn as_arr(&self) -> [f32; 2] {
		return [self.x, self.y];
	}
}

impl Vector3 {
	pub fn as_arr(&self) -> [f32; 3] {
		return [self.x, self.y, self.z];
	}
}

impl Vector4 {
	pub fn as_arr(&self) -> [f32; 4] {
		return [self.x, self.y, self.z, self.w];
	}
}

impl Matrix4 {

	pub fn translate(self, x: f32, y: f32) -> Self {

		let mut m = mat4();

		m.m[3][0] = x;
		m.m[3][1] = y;

		return self * m;

	}

	pub fn scale(self, sx: f32, sy: f32) -> Self {

		let mut m = mat4();

		m.m[0][0] = sx;
		m.m[1][1] = sy;

		return self * m;

	}

	pub fn rotate(self, rot: f32) -> Self {

		let mut m = mat4();

		let c = rot.cos();
		let s = rot.sin();
		let cv = 1.0 - c;

		let axis = vec3(0.0, 0.0, 1.0);

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

		let mut new = mat4();

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

		new.m[0][0] = (self.m[1][1] * f00 - self.m[1][2] * f01 + self.m[1][3] * f02);
		new.m[1][0] = -(self.m[1][0] * f00 - self.m[1][2] * f03 + self.m[1][3] * f04);
		new.m[2][0] = (self.m[1][0] * f01 - self.m[1][1] * f03 + self.m[1][3] * f05);
		new.m[3][0] = -(self.m[1][0] * f02 - self.m[1][1] * f04 + self.m[1][2] * f05);

		new.m[0][1] = -(self.m[0][1] * f00 - self.m[0][2] * f01 + self.m[0][3] * f02);
		new.m[1][1] = (self.m[0][0] * f00 - self.m[0][2] * f03 + self.m[0][3] * f04);
		new.m[2][1] = -(self.m[0][0] * f01 - self.m[0][1] * f03 + self.m[0][3] * f05);
		new.m[3][1] = (self.m[0][0] * f02 - self.m[0][1] * f04 + self.m[0][2] * f05);

		new.m[0][2] = (self.m[0][1] * f06 - self.m[0][2] * f07 + self.m[0][3] * f08);
		new.m[1][2] = -(self.m[0][0] * f06 - self.m[0][2] * f09 + self.m[0][3] * f10);
		new.m[2][2] = (self.m[0][0] * f11 - self.m[0][1] * f09 + self.m[0][3] * f12);
		new.m[3][2] = -(self.m[0][0] * f08 - self.m[0][1] * f10 + self.m[0][2] * f12);

		new.m[0][3] = -(self.m[0][1] * f13 - self.m[0][2] * f14 + self.m[0][3] * f15);
		new.m[1][3] = (self.m[0][0] * f13 - self.m[0][2] * f16 + self.m[0][3] * f17);
		new.m[2][3] = -(self.m[0][0] * f14 - self.m[0][1] * f16 + self.m[0][3] * f18);
		new.m[3][3] = (self.m[0][0] * f15 - self.m[0][1] * f17 + self.m[0][2] * f18);

		let det =
			self.m[0][0] * new.m[0][0] +
			self.m[0][1] * new.m[1][0] +
			self.m[0][2] * new.m[2][0] +
			self.m[0][3] * new.m[3][0];

		for i in 0..4 {
			for j in 0..4 {
				new.m[i][j] *= (1.0 / det);
			}
		}

		return new;

	}

	pub fn forward(&self, pt: Vector2) -> Vector2 {

		let mut npt = vec2(0.0, 0.0);

		npt.x = pt.x * self.m[0][0] + pt.y * self.m[1][0] + self.m[3][0];
		npt.y = pt.x * self.m[0][1] + pt.y * self.m[1][1] + self.m[3][1];

		return npt;

	}

	pub fn as_arr(&self) -> [[f32; 4]; 4] {
		return self.m;
	}

}

impl Mul for Matrix4 {

	type Output = Matrix4;

	fn mul(self, other: Matrix4) -> Matrix4 {

		let mut nm = mat4();

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

pub fn vec2(x: f32, y: f32) -> Vector2 {
	return Vector2 {
		x: x,
		y: y,
	}
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
	return Vector3 {
		x: x,
		y: y,
		z: z,
	}
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
	return Vector4 {
		x: x,
		y: y,
		z: z,
		w: w,
	}
}

pub fn mat4() -> Matrix4 {

	return Matrix4 {
		m: [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		]
	};

}

pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4 {

	let mut m = mat4();

	m.m[0][0] = 2.0 / (right - left);
	m.m[1][1] = 2.0 / (top - bottom);
	m.m[2][2] = 2.0 / (near - far);

	m.m[3][0] = (left + right) / (left - right);
	m.m[3][1] = (bottom + top) / (bottom - top);
	m.m[3][2] = (far + near) / (near - far);

	return m;

}

