// wengwengweng

use crate::math::*;
use super::Texture;

// TODO: is there a way to use &dyn Into<UniformValue>?
pub type UniformValues<'a> = Vec<(&'static str, &'a dyn IntoUniformValue)>;

pub trait IntoUniformValue {
	fn into(&self) -> UniformValue;
}

impl IntoUniformValue for UniformValue {
	fn into(&self) -> UniformValue {
		return *self;
	}
}

pub trait UniformLayout: 'static {
	fn values(&self) -> UniformValues;
	fn texture(&self) -> Option<&dyn Texture>;
}

impl UniformLayout for () {
	fn values(&self) -> Vec<(&'static str, &dyn IntoUniformValue)> {
		return vec![];
	}
	fn texture(&self) -> Option<&dyn Texture> {
		return None;
	}
}

#[derive(Clone, Copy, PartialEq)]
pub enum UniformValue {
	F1(f32),
	F2([f32; 2]),
	F3([f32; 3]),
	F4([f32; 4]),
	Mat4([f32; 16]),
}

impl IntoUniformValue for f32 {
	fn into(&self) -> UniformValue {
		return UniformValue::F1(*self);
	}
}

impl IntoUniformValue for [f32; 2] {
	fn into(&self) -> UniformValue {
		return UniformValue::F2(*self);
	}
}

impl IntoUniformValue for [f32; 3] {
	fn into(&self) -> UniformValue {
		return UniformValue::F3(*self);
	}
}

impl IntoUniformValue for [f32; 4] {
	fn into(&self) -> UniformValue {
		return UniformValue::F4(*self);
	}
}

impl IntoUniformValue for [f32; 16] {
	fn into(&self) -> UniformValue {
		return UniformValue::Mat4(*self);
	}
}

impl IntoUniformValue for Vec2 {
	fn into(&self) -> UniformValue {
		return UniformValue::F2(self.as_arr());
	}
}

impl IntoUniformValue for Vec3 {
	fn into(&self) -> UniformValue {
		return UniformValue::F3(self.as_arr());
	}
}

impl IntoUniformValue for Vec4 {
	fn into(&self) -> UniformValue {
		return UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Color {
	fn into(&self) -> UniformValue {
		return UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Quad {
	fn into(&self) -> UniformValue {
		return UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Mat4 {
	fn into(&self) -> UniformValue {
		return UniformValue::Mat4(self.as_arr());
	}
}

