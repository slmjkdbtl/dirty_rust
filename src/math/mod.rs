// wengwengweng

//! Common Math Functions & Structs

#[macro_use]
mod vec;
mod mat;
#[macro_use]
mod rand;

pub use self::vec::*;
pub use self::mat::*;
pub use self::rand::*;

/// clamp a number within range
pub fn clamp<N: PartialOrd>(x: N, min: N, max: N) -> N {

	if min > max {
		return clamp(x, max, min);
	}

	if x < min {
		return min;
	} else if x > max {
		return max;
	} else {
		return x;
	}

}

/// linear interpolation
pub fn lerp(from: f32, to: f32, amount: f32) -> f32 {
	return from + (to - from) * clamp(amount, 0.0, 1.0);
}

/// cubic interpolation
pub fn smooth(from: f32, to: f32, amount: f32) -> f32 {

	let t = clamp(amount, 0.0, 1.0);
	let m = t * t * (3.0 - 2.0 * t);

	return from + (to - from) * m;

}

/// map a value to another range
pub fn map(val: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
	return clamp(b1 + (val - a1) / (a2 - a1) * (b2 - b1), b1.min(b2), b1.max(b2));
}

/// generate orthographic matrix
pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {

	let mut m = Mat4::identity();

	m.m[0][0] = 2.0 / (right - left);
	m.m[1][1] = 2.0 / (top - bottom);
	m.m[2][2] = 2.0 / (near - far);

	m.m[3][0] = (left + right) / (left - right);
	m.m[3][1] = (bottom + top) / (bottom - top);
	m.m[3][2] = (far + near) / (near - far);

	return m;

}

/// construct perspective matrix
pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {

	let mut m = Mat4::identity();
	let f_depth = far - near;
	let o_depth = 1.0 / f_depth;

	m.m[1][1] = 1.0 / (0.5 * fov).tan();
	m.m[0][0] = m.m[1][1] / aspect;
	m.m[2][2] = far * o_depth;
	m.m[3][2] = (-far * near) * o_depth;
	m.m[2][3] = 1.0;
	m.m[3][3] = 0.0;

	return m;

}

/// construct lookat matrix
pub fn lookat(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {

	let z = (center - eye).unit();
	let x = up.cross(z).unit();
	let y = z.cross(x);
	let mut m = Mat4::identity();

	m.m[0][0] = x.x;
	m.m[0][1] = y.x;
	m.m[0][2] = z.x;

	m.m[1][0] = x.y;
	m.m[1][1] = y.y;
	m.m[1][2] = z.y;

	m.m[2][0] = x.z;
	m.m[2][1] = y.z;
	m.m[2][2] = z.z;

	m.m[3][0] = -x.dot(eye);
	m.m[3][1] = -y.dot(eye);
	m.m[3][2] = -z.dot(eye);

	return m;

}

