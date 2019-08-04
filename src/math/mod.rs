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

	let tx = -(right + left) / (right - left);
	let ty = -(top + bottom) / (top - bottom);
	let tz = -(far + near) / (far - near);

	return Mat4::new([
		2.0 / (right - left), 0.0, 0.0, 0.0,
		0.0, 2.0 / (top - bottom), 0.0, 0.0,
		0.0, 0.0, 2.0 / (near - far), 0.0,
		tx, ty, tz, 1.0,
	]);

}

/// construct perspective matrix
pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {

	let f = 1.0 / (fov / 2.0).tan();

	return Mat4::new([
		f / aspect, 0.0, 0.0, 0.0,
		0.0, f, 0.0, 0.0,
		0.0, 0.0, (far + near) / (far - near), 1.0,
		0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0,
	])

}

/// construct lookat matrix
pub fn lookat(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {

	let z = (center - eye).unit();
	let x = up.cross(z).unit();
	let y = z.cross(x);

	return Mat4::new([
		x.x, y.x, z.x, 0.0,
		x.y, y.y, z.y, 0.0,
		x.z, y.z, z.z, 0.0,
		-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0,
	]);

}

