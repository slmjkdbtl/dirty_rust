// wengwengweng

use super::*;

/// Default Vertex Type
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Vertex {
	pub pos: Vec3,
	pub normal: Vec3,
	pub uv: Vec2,
	pub color: Color,
}

impl VertexLayout for Vertex {
	fn attrs() -> &'static[(&'static str, usize)] {
		return &[
			("a_pos", 3),
			("a_normal", 3),
			("a_uv", 2),
			("a_color", 4),
		];
	}
}

#[derive(Clone, PartialEq)]
pub(super) struct Uniform {
	pub proj: Mat4,
	pub view: Mat4,
	pub model: Mat4,
	pub color: Color,
	pub tex: Texture,
	pub custom: Option<Vec<(&'static str, UniformData)>>,
}

impl UniformLayout for Uniform {

	fn data(&self) -> Vec<(&'static str, UniformData)> {

		let mut values: Vec<(&'static str, UniformData)> = vec![
			("u_proj", UniformData::Mat4(self.proj)),
			("u_view", UniformData::Mat4(self.view)),
			("u_model", UniformData::Mat4(self.model)),
			("u_color", UniformData::Vec4(self.color.as_vec4())),
			("u_tex", UniformData::Texture(self.tex.clone())),
		];

		if let Some(custom) = &self.custom {
			for (name, v) in custom {
				values.push((name, v.clone()));
			}
		}

		return values;

	}

}

