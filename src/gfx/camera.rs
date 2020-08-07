// wengwengweng

use super::*;

/// Describes Features of a Camera
pub trait Camera {
	/// projection matrix
	fn proj(&self) -> Mat4;
	/// view matrix
	fn view(&self) -> Mat4;
	// TODO: test implementations of items below
	/// convert a 2d point to a ray
	fn pt_to_ray(&self, ctx: &Gfx, pt: Vec2) -> Ray3;
	/// convert a point into 2d screen space
	fn to_screen(&self, ctx: &Gfx, pt: Vec3) -> Vec2 {
		let cp = self.proj() * self.view() * vec4!(pt.x, pt.y, pt.z, 1.0);
		let cp = cp.xy() / cp.w;
		return ctx.clip_to_screen(cp);
	}
}

/// Perspective Camera
#[derive(Clone, Debug)]
pub struct PerspectiveCam {
	pub up: Vec3,
	pub dir: Vec3,
	pub pos: Vec3,
	pub fov: f32,
	pub aspect: f32,
	pub near: f32,
	pub far: f32,
}

impl PerspectiveCam {

	/// set pitch / yaw angle
	pub fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.dir = vec3!(
			pitch.cos() * (yaw - f32::to_radians(90.0)).cos(),
			pitch.sin(),
			pitch.cos() * (yaw - f32::to_radians(90.0)).sin(),
		).unit();

	}

	/// set roll angle
	pub fn set_roll(&mut self, roll: f32) {
		let xy = Vec2::from_angle(roll + f32::to_radians(90.0));
		self.up = vec3!(xy.x, xy.y, 0.0);
	}

	/// set destination
	pub fn set_dest(&mut self, l: Vec3) {
		self.dir = (l - self.pos).unit();
	}

	/// get yaw angle
	pub fn yaw(&self) -> f32 {
		return f32::atan2(self.dir.z, self.dir.x) + f32::to_radians(90.0);
	}

	/// get pitch angle
	pub fn pitch(&self) -> f32 {
		return self.dir.y.asin();
	}

	/// get roll angle
	pub fn roll(&self) -> f32 {
		return f32::atan2(self.up.y, self.up.x) - f32::to_radians(90.0);
	}

	/// get front dir
	pub fn front(&self) -> Vec3 {
		return self.dir;
	}

	/// get back dir
	pub fn back(&self) -> Vec3 {
		return -self.dir;
	}

	/// get left dir
	pub fn left(&self) -> Vec3 {
		return -Vec3::cross(self.dir, vec3!(0, 1, 0)).unit();
	}

	/// get right dir
	pub fn right(&self) -> Vec3 {
		return Vec3::cross(self.dir, vec3!(0, 1, 0)).unit();
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

		let z = self.dir.unit();
		let x = Vec3::cross(self.up, z).unit();
		let y = Vec3::cross(z, x);

		return mat4!(
			x.x, y.x, z.x, 0.0,
			x.y, y.y, z.y, 0.0,
			x.z, y.z, z.z, 0.0,
			-Vec3::dot(x, self.pos), -Vec3::dot(y, self.pos), -Vec3::dot(z, self.pos), 1.0,
		);

	}

	fn pt_to_ray(&self, ctx: &Gfx, pt: Vec2) -> Ray3 {

		let ndc = ctx.screen_to_clip(pt);
		let ray_clip = vec4!(ndc.x, ndc.y, 1.0, 1.0);
		let ray_eye = self.proj().inverse() * ray_clip;
		let ray_eye = vec4!(ray_eye.x, ray_eye.y, 1.0, 0.0);
		let ray_wor = (self.view().inverse() * ray_eye).xyz().unit();

		return Ray3 {
			origin: self.pos,
			dir: ray_wor,
		};

	}

}

/// Orthographic Camera
#[derive(Clone, Debug)]
pub struct OrthoCam {
	pub width: f32,
	pub height: f32,
	pub near: f32,
	pub far: f32,
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

	fn pt_to_ray(&self, ctx: &Gfx, pt: Vec2) -> Ray3 {

		let dir = vec3!(0, 0, -1);

		let normalized = ctx.screen_to_clip(pt);
		let clip_coord = vec4!(normalized.x, normalized.y, -1, 1);
		let orig = self.proj().inverse() * clip_coord;

		return Ray3::new(orig.xyz(), vec3!(dir.x, -dir.y, dir.z));

	}

}

/// Oblique Camera
#[derive(Clone, Debug)]
pub struct ObliqueCam {
	pub width: f32,
	pub height: f32,
	pub near: f32,
	pub far: f32,
	pub angle: f32,
	pub z_scale: f32,
}

impl ObliqueCam {

	fn ortho(&self) -> Mat4 {

		return OrthoCam {
			width: self.width,
			height: self.height,
			near: self.near,
			far: self.far,
		}.proj();

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
		return self.ortho() * self.skew();
	}

	fn view(&self) -> Mat4 {
		return mat4!();
	}

	fn pt_to_ray(&self, ctx: &Gfx, pt: Vec2) -> Ray3 {

		let dir = (self.skew() * vec3!(0, 0, -1)).unit();

		let normalized = ctx.screen_to_clip(pt);
		let clip_coord = vec4!(normalized.x, normalized.y, -1, 1);
		let orig = self.proj().inverse() * clip_coord;

		return Ray3::new(orig.xyz(), vec3!(-dir.x, -dir.y, dir.z));

	}

}

/// Camera from Raw Proj & View Matrices
pub struct RawCam {
	pub proj: Mat4,
	pub view: Mat4,
}

impl Camera for RawCam {

	fn proj(&self) -> Mat4 {
		return self.proj;
	}

	fn view(&self) -> Mat4 {
		return self.view;
	}

	fn pt_to_ray(&self, ctx: &Gfx, pt: Vec2) -> Ray3 {

		let dir = vec3!(0, 0, -1);

		let normalized = ctx.screen_to_clip(pt);
		let clip_coord = vec4!(normalized.x, normalized.y, -1, 1);
		let orig = self.proj().inverse() * clip_coord;

		return Ray3::new(orig.xyz(), vec3!(dir.x, -dir.y, dir.z));

	}

}

