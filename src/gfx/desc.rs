// wengwengweng

use serde::Serialize;
use serde::Deserialize;

use crate::*;
use math::*;
use gfx::*;

/// Mesh Data with Vertices & Indices
#[derive(Clone, Serialize, Deserialize)]
pub struct MeshData {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,
}

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
pub(crate) struct Uniform {
	pub proj: Mat4,
	pub view: Mat4,
	pub model: Mat4,
	pub color: Color,
	pub tex: Texture,
	pub custom: Option<Vec<(&'static str, UniformValue)>>,
}

impl UniformLayout for Uniform {

	fn values(&self) -> UniformValues {

		let mut values: UniformValues = hmap![
			"u_proj" => &self.proj,
			"u_view" => &self.view,
			"u_model" => &self.model,
			"u_color" => &self.color,
		];

		if let Some(custom) = &self.custom {
			for (name, v) in custom {
				values.insert(name, v);
			}
		}

		return values;

	}

	fn textures(&self) -> Vec<&Texture> {
		return vec![&self.tex];
	}

}

