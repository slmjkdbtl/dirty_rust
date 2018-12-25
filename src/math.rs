// wengwengweng

use core::ops::Mul;

pub struct Vector2 {
	x: f32,
	y: f32,
}

pub struct Vector3 {
	x: f32,
	y: f32,
	z: f32,
}

pub struct Vector4 {
	x: f32,
	y: f32,
	z: f32,
	w: f32,
}

pub struct Matrix4 {
	m: [[f32; 4]; 4],
}

impl Vector2 {
	pub fn arr(&self) -> [f32; 2] {
		return [self.x, self.y];
	}
}

impl Vector3 {
	pub fn arr(&self) -> [f32; 3] {
		return [self.x, self.y, self.z];
	}
}

impl Vector4 {
	pub fn arr(&self) -> [f32; 4] {
		return [self.x, self.y, self.z, self.w];
	}
}

impl Matrix4 {

	pub fn translate(self, x: f32, y: f32) -> Matrix4 {

		let mut m = mat4();

		m.m[3][0] = x;
		m.m[3][1] = y;

		return self * m;

	}

	pub fn scale(self, sx: f32, sy: f32) -> Matrix4 {

		let mut m = mat4();

		m.m[0][0] = sx;
		m.m[1][1] = sy;

		return self * m;

	}

	pub fn rotate(self, rot: f32) -> Matrix4 {

		let mut m = mat4();

		return self * m;

	}

	pub fn matrix(&self) -> [[f32; 4]; 4] {
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

