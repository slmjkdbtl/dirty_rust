// wengwengweng

use crate::*;
use super::*;

pub trait Camera {
	fn projection(&self) -> Mat4;
	fn lookat(&self) -> Mat4;
}

#[derive(Clone)]
pub struct PerspectiveCam {
	front: Vec3,
	pos: Vec3,
	yaw: f32,
	pitch: f32,
	fov: f32,
	aspect: f32,
	near: f32,
	far: f32,
}

impl PerspectiveCam {

	pub fn new(fov: f32, aspect: f32, near: f32, far: f32, pos: Vec3, yaw: f32, pitch: f32) -> Self {

		let mut c = Self {
			pos: vec3!(),
			front: vec3!(),
			yaw: 0.0,
			pitch: 0.0,
			fov: fov,
			aspect: aspect,
			near: near,
			far: far,
		};

		c.set_pos(pos);
		c.set_angle(yaw, pitch);

		return c;

	}

	pub fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
	}

	pub fn set_front(&mut self, front: Vec3) {
		// TODO: calculate yaw & pitch from front
		self.front = front;
	}

	pub fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.yaw = yaw;
		self.pitch = pitch;

		self.front = vec3!(
			self.pitch.cos() * (self.yaw - 90f32.to_radians()).cos(),
			self.pitch.sin(),
			self.pitch.cos() * (self.yaw - 90f32.to_radians()).sin(),
		).normalize();

	}

	pub fn front(&self) -> Vec3 {
		return self.front;
	}

	pub fn yaw(&self) -> f32 {
		return self.yaw;
	}

	pub fn pitch(&self) -> f32 {
		return self.pitch;
	}

	pub fn pos(&self) -> Vec3 {
		return self.pos;
	}

}

impl Camera for PerspectiveCam {

	fn projection(&self) -> Mat4 {

		let f = 1.0 / (self.fov / 2.0).tan();

		return mat4!(
			-f / self.aspect, 0.0, 0.0, 0.0,
			0.0, f, 0.0, 0.0,
			0.0, 0.0, (self.far + self.near) / (self.far - self.near), 1.0,
			0.0, 0.0, -(2.0 * self.far * self.near) / (self.far - self.near), 0.0,
		);

	}

	fn lookat(&self) -> Mat4 {
		return lookat(self.pos, self.pos + self.front, vec3!(0, 1, 0));
	}

}

#[derive(Clone)]
pub struct OrthoCam {
	pub width: f32,
	pub height: f32,
	pub near: f32,
	pub far: f32,
}

impl OrthoCam {

	pub fn new(width: f32, height: f32, near: f32, far: f32) -> Self {

		return Self {
			width: width,
			height: height,
			near: near,
			far: far,
		};

	}

}

impl Camera for OrthoCam {

	fn projection(&self) -> Mat4 {

		let w = self.width;
		let h = self.height;
		let near = self.near;
		let far = self.far;

		let (left, right, bottom, top) = (-w / 2.0, w / 2.0, -h / 2.0, h / 2.0);
		let tx = -(right + left) / (right - left);
		let ty = -(top + bottom) / (top - bottom);
		let tz = -(far + near) / (far - near);

		return Mat4::new([
			2.0 / (right - left), 0.0, 0.0, 0.0,
			0.0, 2.0 / (top - bottom), 0.0, 0.0,
			0.0, 0.0, -2.0 / (far - near), 0.0,
			tx, ty, tz, 1.0,
		]);

	}

	fn lookat(&self) -> Mat4 {
		return mat4!();
	}

}

fn lookat(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {

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

