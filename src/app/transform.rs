// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug)]
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

