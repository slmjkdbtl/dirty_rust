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
	pub custom: Option<UniformData>,
}

impl UniformLayout for Uniform {

	fn values(&self) -> Vec<(&'static str, UniformValue)> {

		let mut values: Vec<(&'static str, UniformValue)> = vec![
			("u_proj", UniformValue::Mat4(self.proj)),
			("u_view", UniformValue::Mat4(self.view)),
			("u_model", UniformValue::Mat4(self.model)),
			("u_color", UniformValue::Vec4(self.color.as_vec4())),
		];

		if let Some(custom) = &self.custom {
			for (name, v) in &custom.values {
				values.push((name, *v));
			}
		}

		return values;

	}

	fn textures(&self) -> Vec<&Texture> {
		let mut textures = vec![&self.tex];
		if let Some(custom) = &self.custom {
			textures.extend(custom.textures.iter());
		}
		return textures;
	}

}

