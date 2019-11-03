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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
	Right,
	Down,
	Left,
	Up,
}

impl Dir {
	pub fn as_vec2(&self) -> Vec2 {
		return match self {
			Dir::Right => vec2!(1, 0),
			Dir::Down => vec2!(0, 1),
			Dir::Left => vec2!(-1, 0),
			Dir::Up => vec2!(0, -1),
		};
	}
}

pub trait Lerpable =
	Copy
	+ Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<f32, Output = Self>
	;

pub trait Lerp: Lerpable {

	/// linear interpolation
	fn lerp(self, to: Self, amount: f32) -> Self {
		return self + (to - self) * amount.clamp(0.0, 1.0);
	}

	/// cubic interpolation
	fn smooth(self, to: Self, amount: f32) -> Self {

		let t = amount.clamp(0.0, 1.0);
		let m = t * t * (3.0 - 2.0 * t);

		return self + (to - self) * m;

	}

}

impl<T: Lerpable> Lerp for T {}

pub trait Mappable =
	Copy
	+ Add<Self, Output = Self>
	+ Sub<Self, Output = Self>
	+ Mul<Self, Output = Self>
	+ Div<Self, Output = Self>
	;

pub trait Map: Mappable {

	/// map a value to another range
	fn map(self, a1: Self, a2: Self, b1: Self, b2: Self) -> Self {
		return b1 + (self - a1) / (a2 - a1) * (b2 - b1);
	}

}

impl<T: Mappable> Map for T {}

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
		-f / aspect, 0.0, 0.0, 0.0,
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

