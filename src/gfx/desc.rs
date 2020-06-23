// wengwengweng

use serde::Serialize;
use serde::Deserialize;

use crate::*;
use gfx::*;

/// Default Vertex Type
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Vertex {
	pub pos: Vec3,
	pub uv: Vec2,
	pub normal: Vec3,
	pub color: Color,
}

impl VertexLayout for Vertex {
	fn attrs() -> VertexAttrGroup {
		return &[
			("a_pos", 3),
			("a_uv", 2),
			("a_normal", 3),
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

	fn values(&self) -> UniformValues {

		let mut values: UniformValues = vec![
			("u_proj", &self.proj),
			("u_view", &self.view),
			("u_model", &self.model),
			("u_color", &self.color),
		];

		if let Some(custom) = &self.custom {
			for (name, v) in &custom.values {
				values.push((name, v));
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

