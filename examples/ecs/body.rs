// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

comp!(Body {

	verts: Vec<Vec2>,
	d_verts: Vec<Vec2>,

});

impl Body {

	pub fn new() -> Self {

		return Self {
			verts: Vec::new(),
			d_verts: Vec::new(),
		};

	}

	pub fn from_verts(verts: &[Vec2]) -> Self {

		return Self {
			verts: verts.to_vec(),
			d_verts: verts.to_vec(),
		};

	}

}

