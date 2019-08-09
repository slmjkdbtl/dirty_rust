// wengwengweng

use crate::math::*;

pub enum UniformType {
	F1(f32),
	F2(f32, f32),
	F3(f32, f32, f32),
	F4(f32, f32, f32, f32),
	Mat4([f32; 16]),
}

pub trait UniformValue {
	fn get(&self) -> UniformType;
}

impl UniformValue for f32 {
	fn get(&self) -> UniformType {
		return UniformType::F1(*self);
	}
}

impl UniformValue for [f32; 2] {
	fn get(&self) -> UniformType {
		return UniformType::F2(self[0], self[1]);
	}
}

impl UniformValue for [f32; 3] {
	fn get(&self) -> UniformType {
		return UniformType::F3(self[0], self[1], self[2]);
	}
}

impl UniformValue for [f32; 4] {
	fn get(&self) -> UniformType {
		return UniformType::F4(self[0], self[1], self[2], self[3]);
	}
}

impl UniformValue for Vec2 {
	fn get(&self) -> UniformType {
		return UniformType::F2(self.x, self.y);
	}
}

impl UniformValue for Vec3 {
	fn get(&self) -> UniformType {
		return UniformType::F3(self.x, self.y, self.z);
	}
}

impl UniformValue for Vec4 {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.x, self.y, self.z, self.w);
	}
}

impl UniformValue for Color {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.r, self.g, self.b, self.a);
	}
}

impl UniformValue for Quad {
	fn get(&self) -> UniformType {
		return UniformType::F4(self.x, self.y, self.w, self.h);
	}
}

impl UniformValue for Mat4 {
	fn get(&self) -> UniformType {
		return UniformType::Mat4(self.as_arr());
	}
}


