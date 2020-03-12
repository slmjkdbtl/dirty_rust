// wengwengweng

use crate::*;
use math::*;

pub trait Camera {
	fn proj(&self) -> Mat4;
	fn view(&self) -> Mat4;
	fn mouse_ray(&self, ctx: &Ctx) -> Ray3;
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
		).normalized();

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

	fn proj(&self) -> Mat4 {

		let f = 1.0 / (self.fov / 2.0).tan();

		return mat4!(
			-f / self.aspect, 0.0, 0.0, 0.0,
			0.0, f, 0.0, 0.0,
			0.0, 0.0, (self.far + self.near) / (self.far - self.near), 1.0,
			0.0, 0.0, -(2.0 * self.far * self.near) / (self.far - self.near), 0.0,
		);

	}

	fn view(&self) -> Mat4 {

		let eye = self.pos;
		let center = self.pos + self.front;
		let up = vec3!(0, 1, 0);
		let z = (center - eye).normalized();
		let x = up.cross(z).normalized();
		let y = z.cross(x);

		return mat4!(
			x.x, y.x, z.x, 0.0,
			x.y, y.y, z.y, 0.0,
			x.z, y.z, z.z, 0.0,
			-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0,
		);

	}

	// TODO
	fn mouse_ray(&self, ctx: &Ctx) -> Ray3 {
		todo!();
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

	fn proj(&self) -> Mat4 {

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

	fn view(&self) -> Mat4 {
		return mat4!();
	}

	// TODO
	fn mouse_ray(&self, ctx: &Ctx) -> Ray3 {

		let dir = vec3!(0, 0, -1);

		let normalized = ctx.screen_to_clip(ctx.mouse_pos());
		let clip_coord = vec4!(normalized.x, normalized.y, -1, 1);
		let orig = self.proj().inverse() * clip_coord;

		return Ray3::new(orig.xyz(), vec3!(dir.x, -dir.y, dir.z));

	}

}

#[derive(Clone)]
pub struct ObliqueCam {
	pub width: f32,
	pub height: f32,
	pub near: f32,
	pub far: f32,
	pub angle: f32,
	pub z_scale: f32,
}

impl ObliqueCam {

	pub fn new(angle: f32, z_scale: f32, width: f32, height: f32, near: f32, far: f32) -> Self {

		return Self {
			width: width,
			height: height,
			near: near,
			far: far,
			angle: angle,
			z_scale: z_scale,
		};

	}

	fn ortho(&self) -> Mat4 {

		let w = self.width;
		let h = self.height;
		let near = self.near;
		let far = self.far;

		let (left, right, bottom, top) = (-w / 2.0, w / 2.0, -h / 2.0, h / 2.0);
		let tx = -(right + left) / (right - left);
		let ty = -(top + bottom) / (top - bottom);
		let tz = -(far + near) / (far - near);

		return mat4![
			2.0 / (right - left), 0.0, 0.0, 0.0,
			0.0, 2.0 / (top - bottom), 0.0, 0.0,
			0.0, 0.0, -2.0 / (far - near), 0.0,
			tx, ty, tz, 1.0,
		];

	}

	fn skew(&self) -> Mat4 {

		let a = -self.z_scale * f32::cos(self.angle);
		let b = -self.z_scale * f32::sin(self.angle);

		return mat4![
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			a, b, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0,
		];

	}

}

impl Camera for ObliqueCam {

	fn proj(&self) -> Mat4 {

		let ortho = OrthoCam {
			width: self.width,
			height: self.height,
			near: self.near,
			far: self.far,
		};

		return ortho.proj() * self.skew();
	}

	fn view(&self) -> Mat4 {
		return mat4!();
	}

	fn mouse_ray(&self, ctx: &Ctx) -> Ray3 {

		let dir = (self.skew() * vec3!(0, 0, -1)).normalized();

		let normalized = ctx.screen_to_clip(ctx.mouse_pos());
		let clip_coord = vec4!(normalized.x, normalized.y, -1, 1);
		let orig = self.proj().inverse() * clip_coord;

		return Ray3::new(orig.xyz(), vec3!(dir.x, -dir.y, dir.z));

	}

}

