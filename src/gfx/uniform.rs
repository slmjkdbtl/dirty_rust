// wengwengweng

use super::*;

/// Trait for Custom Uniform Data. See [mod-level doc](index.html) for Usage.
pub trait UniformLayout {
	fn data(&self) -> Vec<(&'static str, UniformData)> {
		return vec![];
	}
}

impl UniformLayout for () {}

#[derive(Clone, PartialEq)]
pub enum UniformData {
	Float(f32),
	Vec2(Vec2),
	Vec3(Vec3),
	Vec4(Vec4),
	Int(i32),
	Mat4(Mat4),
	Texture(Texture),
}

