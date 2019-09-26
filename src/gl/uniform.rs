// wengwengweng

use crate::math::*;
use super::Texture;

// TODO: wait for impl Trait in Traits
#[derive(Clone, PartialEq)]
pub struct UniformValues {
	pub(super) values: Vec<(&'static str, UniformType)>,
	pub(super) texture: Option<Texture>,
}

impl UniformValues {
	pub fn build() -> Self {
		return Self {
			values: vec![],
			texture: None,
		};
	}
	pub fn value(mut self, name: &'static str, val: impl UniformValue) -> Self {
		self.values.push((name, val.as_uniform()));
		return self;
	}
	pub fn texture(mut self, tex: &Texture) -> Self {
		self.texture = Some(tex.clone());
		return self;
	}
}

pub trait UniformInterface: 'static {
	fn send(&self) -> UniformValues;
}

#[derive(Clone, PartialEq)]
pub enum UniformType {
	F1(f32),
	F2(f32, f32),
	F3(f32, f32, f32),
	F4(f32, f32, f32, f32),
	I1(i32),
	I2(i32, i32),
	I3(i32, i32, i32),
	I4(i32, i32, i32, i32),
	Mat4([f32; 16]),
}

pub trait UniformValue {
	fn as_uniform(&self) -> UniformType;
}

impl UniformValue for f32 {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F1(*self);
	}
}

impl UniformValue for [f32; 2] {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F2(self[0], self[1]);
	}
}

impl UniformValue for [f32; 3] {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F3(self[0], self[1], self[2]);
	}
}

impl UniformValue for [f32; 4] {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F4(self[0], self[1], self[2], self[3]);
	}
}

impl UniformValue for i32 {
	fn as_uniform(&self) -> UniformType {
		return UniformType::I1(*self);
	}
}

impl UniformValue for [i32; 2] {
	fn as_uniform(&self) -> UniformType {
		return UniformType::I2(self[0], self[1]);
	}
}

impl UniformValue for [i32; 3] {
	fn as_uniform(&self) -> UniformType {
		return UniformType::I3(self[0], self[1], self[2]);
	}
}

impl UniformValue for [i32; 4] {
	fn as_uniform(&self) -> UniformType {
		return UniformType::I4(self[0], self[1], self[2], self[3]);
	}
}

impl UniformValue for Vec2 {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F2(self.x, self.y);
	}
}

impl UniformValue for Vec3 {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F3(self.x, self.y, self.z);
	}
}

impl UniformValue for Vec4 {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F4(self.x, self.y, self.z, self.w);
	}
}

impl UniformValue for Color {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F4(self.r, self.g, self.b, self.a);
	}
}

impl UniformValue for Quad {
	fn as_uniform(&self) -> UniformType {
		return UniformType::F4(self.x, self.y, self.w, self.h);
	}
}

impl UniformValue for Mat4 {
	fn as_uniform(&self) -> UniformType {
		return UniformType::Mat4(self.as_arr());
	}
}


