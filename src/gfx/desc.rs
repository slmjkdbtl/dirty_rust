// wengwengweng

use serde::Serialize;
use serde::Deserialize;

use crate::*;
use math::*;
use gfx::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct MeshData {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Vertex {
	pub pos: Vec3,
	pub uv: Vec2,
	pub normal: Vec3,
	pub color: Color,
}

impl gl::VertexLayout for Vertex {
	fn attrs() -> gl::VertexAttrGroup {
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
	pub custom: Option<Vec<(&'static str, gl::UniformValue)>>,
}

impl gl::UniformLayout for Uniform {

	fn values(&self) -> gl::UniformValues {

		let mut values: gl::UniformValues = hmap![
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

	fn textures(&self) -> Vec<&dyn gl::Texture> {
		return vec![self.tex.gl_tex()];
	}

}

