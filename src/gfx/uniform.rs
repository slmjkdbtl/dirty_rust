// wengwengweng

use super::*;

pub type UniformValues<'a> = Vec<(&'static str, &'a dyn IntoUniformValue)>;

pub trait IntoUniformValue {
	fn into_uniform(&self) -> UniformValue;
}

impl IntoUniformValue for UniformValue {
	fn into_uniform(&self) -> UniformValue {
		return *self;
	}
}

/// Trait for Custom Uniform Data. See [mod-level doc](index.html) for Usage.
pub trait UniformLayout {
	fn values(&self) -> UniformValues {
		return vec![];
	}
	fn textures(&self) -> Vec<&Texture> {
		return vec![];
	}
}

impl UniformLayout for () {}

#[derive(Clone, PartialEq)]
pub(super) struct UniformData {
	pub values: Vec<(&'static str, UniformValue)>,
	pub textures: Vec<Texture>,
}

impl UniformData {
	pub fn from_uniform(uniform: &impl UniformLayout) -> Self {
		return Self {
			values: uniform
				.values()
				.into_iter()
				.map(|(n, v)| (n, v.into_uniform()))
				.collect(),
			textures: uniform
				.textures()
				.into_iter()
				.cloned()
				.collect(),
		};
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

impl IntoUniformValue for Vec2 {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F2(self.as_arr());
	}
}

impl IntoUniformValue for Vec3 {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F3(self.as_arr());
	}
}

impl IntoUniformValue for Vec4 {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Color {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Quad {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F4(self.as_arr());
	}
}

impl IntoUniformValue for Mat4 {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::Mat4(self.as_arr());
	}
}

impl IntoUniformValue for std::time::Duration {
	fn into_uniform(&self) -> UniformValue {
		return UniformValue::F1(self.as_secs_f32());
	}
}

