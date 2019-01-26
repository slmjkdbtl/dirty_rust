// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::addons::ecs::*;

comp!(Body {

	verts: Vec<Vec2>,
	d_verts: Vec<Vec2>,

});

impl Body {

	pub fn new(verts: &[Vec2]) -> Self {

		return Self {
			verts: verts.to_vec(),
			d_verts: verts.to_vec(),
		};

	}

}

