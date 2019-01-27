// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::ecs::*;
use dirty::ecs::derive::*;

#[derive(Comp, Clone)]
pub struct Body {

	pub verts: Vec<Vec2>,
	pub d_verts: Vec<Vec2>,

}

impl Body {

	pub fn new(verts: &[Vec2]) -> Self {

		return Self {
			verts: verts.to_vec(),
			d_verts: verts.to_vec(),
		};

	}

}

