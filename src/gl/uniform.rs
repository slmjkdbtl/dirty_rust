// wengwengweng

use std::collections::HashMap;
use super::Texture;

// TODO: is there a way to use &dyn Into<UniformValue>?
pub type UniformValues<'a> = HashMap<&'static str, &'a dyn IntoUniformValue>;

pub trait IntoUniformValue {
	fn into_uniform(&self) -> UniformValue;
}

impl IntoUniformValue for UniformValue {
	fn into_uniform(&self) -> UniformValue {
		return *self;
	}
}

pub trait UniformLayout: 'static {
	fn values(&self) -> UniformValues;
	fn texture(&self) -> Option<&dyn Texture>;
}

impl UniformLayout for () {
	fn values(&self) -> UniformValues {
		return hashmap![];
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
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F1(*self);
	}
}

impl IntoUniformValue for [f32; 2] {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F2(*self);
	}
}

impl IntoUniformValue for [f32; 3] {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F3(*self);
	}
}

impl IntoUniformValue for [f32; 4] {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F4(*self);
	}
}

impl IntoUniformValue for [f32; 16] {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::Mat4(*self);
	}
}

