// wengwengweng

use super::Texture;

pub type UniformValues = Vec<(&'static str, UniformType)>;

// TODO: wait for impl Trait in Traits
pub trait UniformInterface: 'static {
	fn values(&self) -> UniformValues;
	fn texture(&self) -> Option<&Texture>;
}

#[derive(Clone, PartialEq)]
pub enum UniformType {
	F1(f32),
	F2([f32; 2]),
	F3([f32; 3]),
	F4([f32; 4]),
	Mat4([f32; 16]),
}

