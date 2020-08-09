// wengwengweng

use super::*;

/// Trait for Custom Uniform Data. See [mod-level doc](index.html) for Usage.
pub trait UniformLayout {
	fn values(&self) -> Vec<(&'static str, UniformValue)> {
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
				.map(|(n, v)| (n, v))
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
	Float(f32),
	Vec2(Vec2),
	Vec3(Vec3),
	Vec4(Vec4),
	Int(i32),
	Mat4(Mat4),
}

