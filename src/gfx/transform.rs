// wengwengweng

use super::*;

/// Transform Data for 3D Objects
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform {
	pub pos: Vec3,
	pub rot: Vec4,
	pub scale: Vec3,
}

impl Transform {

	pub fn new() -> Self {
		return Self {
			pos: vec3!(0),
			rot: vec4!(0, 0, 0, 1),
			scale: vec3!(1),
		}
	}

	pub fn as_mat4(&self) -> Mat4 {
		return mat4!()
			.t3(self.pos)
			.s3(self.scale)
			.rq(self.rot)
			;
	}

}

impl Mat4 {

	pub fn t3(self, p: Vec3) -> Self {
		return self * Self::translate(p);
	}

	pub fn t2(self, p: Vec2) -> Self {
		return self.t3(vec3!(p.x, p.y, 0.0));
	}

	pub fn tx(self, dx: f32) -> Self {
		return self.t3(vec3!(dx, 0, 0));
	}

	pub fn ty(self, dy: f32) -> Self {
		return self.t3(vec3!(0, dy, 0));
	}

	pub fn tz(self, dz: f32) -> Self {
		return self.t3(vec3!(0, 0, dz));
	}

	pub fn r(self, angle: f32) -> Self {
		return self *  Self::rotate_z(angle);
	}

	pub fn rx(self, angle: f32) -> Self {
		return self *  Self::rotate_x(angle);
	}

	pub fn ry(self, angle: f32) -> Self {
		return self * Self::rotate_y(angle);
	}

	pub fn rz(self, angle: f32) -> Self {
		return self *  Self::rotate_z(angle);
	}

	pub fn rq(self, q: Vec4) -> Self {
		return self *  Self::rotate_quat(q);
	}

	pub fn rd(self, d: Vec3) -> Self {
		return self *  Self::rotate_dir(d);
	}

	pub fn s3(self, s: Vec3) -> Self {
		return self * Self::scale(s);
	}

	pub fn s2(&self, s: Vec2) -> Self {
		return self.s3(vec3!(s.x, s.y, 1.0));
	}

	pub fn sx(&self, x: f32) -> Self {
		return self.s3(vec3!(x, 1, 1));
	}

	pub fn sy(&self, y: f32) -> Self {
		return self.s3(vec3!(1, y, 1));
	}

	pub fn sz(&self, z: f32) -> Self {
		return self.s3(vec3!(1, 1, z));
	}

	pub fn sk3(self, s: Vec3) -> Self {
		return self * Self::skew(s);
	}

	pub fn sk2(&self, s: Vec2) -> Self {
		return self.sk3(vec3!(s.x, s.y, 0));
	}

	pub fn skx(&self, x: f32) -> Self {
		return self.sk2(vec2!(x, 0));
	}

	pub fn sky(&self, y: f32) -> Self {
		return self.sk2(vec2!(0, y));
	}

}

