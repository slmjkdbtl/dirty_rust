// wengwengweng

use crate::math::*;
use super::Texture;

pub type UniformValues = Vec<(&'static str, UniformType)>;

// TODO: -> impl UniformValue
pub trait UniformLayout: 'static {
	fn values(&self) -> UniformValues;
	fn texture(&self) -> Option<&Texture>;
}

impl UniformLayout for () {
	fn values(&self) -> UniformValues {
		return vec![];
	}
	fn texture(&self) -> Option<&Texture> {
		return None;
	}
}

#[derive(Clone, PartialEq)]
pub enum UniformType {
	F1(f32),
	F2([f32; 2]),
	F3([f32; 3]),
	F4([f32; 4]),
	Mat4([f32; 16]),
}

impl From<f32> for UniformType {
	fn from(v: f32) -> Self {
		return Self::F1(v);
	}
}

impl From<[f32; 2]> for UniformType {
	fn from(v: [f32; 2]) -> Self {
		return Self::F2(v);
	}
}

impl From<[f32; 3]> for UniformType {
	fn from(v: [f32; 3]) -> Self {
		return Self::F3(v);
	}
}

impl From<[f32; 4]> for UniformType {
	fn from(v: [f32; 4]) -> Self {
		return Self::F4(v);
	}
}

impl From<[f32; 16]> for UniformType {
	fn from(v: [f32; 16]) -> Self {
		return Self::Mat4(v);
	}
}

impl From<Vec2> for UniformType {
	fn from(v: Vec2) -> Self {
		return Self::F2(v.as_arr());
	}
}

impl From<Vec3> for UniformType {
	fn from(v: Vec3) -> Self {
		return Self::F3(v.as_arr());
	}
}

impl From<Vec4> for UniformType {
	fn from(v: Vec4) -> Self {
		return Self::F4(v.as_arr());
	}
}

impl From<Color> for UniformType {
	fn from(v: Color) -> Self {
		return Self::F4(v.as_arr());
	}
}

impl From<Quad> for UniformType {
	fn from(v: Quad) -> Self {
		return Self::F4(v.as_arr());
	}
}

impl From<Mat4> for UniformType {
	fn from(m: Mat4) -> Self {
		return Self::Mat4(m.as_arr());
	}
}

