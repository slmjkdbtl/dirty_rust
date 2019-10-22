// wengwengweng

//! Common Math Functions & Structs

mod vec;
#[macro_use]
mod mat;
#[macro_use]
mod rand;

use std::ops::*;

pub use vec::*;
pub use mat::*;
pub use rand::*;

pub trait Lerpable =
	Copy
	+ Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<f32, Output = Self>
	;

/// linear interpolation
pub fn lerp<N: Lerpable>(from: N, to: N, amount: f32) -> N {
	return from + (to - from) * amount.clamp(0.0, 1.0);
}

/// cubic interpolation
pub fn smooth<N: Lerpable>(from: N, to: N, amount: f32) -> N {

	let t = amount.clamp(0.0, 1.0);
	let m = t * t * (3.0 - 2.0 * t);

	return from + (to - from) * m;

}

pub trait Mappable =
	Copy
	+ Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<Output = Self>
	+ Div<Output = Self>
	;

/// map a value to another range
pub fn map<N: Mappable>(val: N, a1: N, a2: N, b1: N, b2: N) -> N {
	return b1 + (val - a1) / (a2 - a1) * (b2 - b1);
}

/// generate orthographic matrix
pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {

	let tx = -(right + left) / (right - left);
	let ty = -(top + bottom) / (top - bottom);
	let tz = -(far + near) / (far - near);

	return mat4!(
		2.0 / (right - left), 0.0, 0.0, 0.0,
		0.0, 2.0 / (top - bottom), 0.0, 0.0,
		0.0, 0.0, 2.0 / (near - far), 0.0,
		tx, ty, tz, 1.0,
	);

}

/// construct perspective matrix
pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {

	let f = 1.0 / (fov / 2.0).tan();

	return mat4!(
		f / aspect, 0.0, 0.0, 0.0,
		0.0, f, 0.0, 0.0,
		0.0, 0.0, (far + near) / (far - near), 1.0,
		0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0,
	);

}

/// construct lookat matrix
pub fn lookat(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {

	let z = (center - eye).normalize();
	let x = up.cross(z).normalize();
	let y = z.cross(x);

	return mat4!(
		x.x, y.x, z.x, 0.0,
		x.y, y.y, z.y, 0.0,
		x.z, y.z, z.z, 0.0,
		-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0,
	);

}

