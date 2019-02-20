// wengwengweng

use std::collections::HashMap;

use dirty::*;
use dirty::math::*;
use dirty::kit::*;

#[derive(Clone)]
pub struct Body {

	pub verts: Vec<Vec2>,
	pub d_verts: Vec<Vec2>,
	pub col: HashMap<Id, Vec2>,
	pub filter: CompFilter,

}

impl Body {

	pub fn new(verts: &[Vec2]) -> Self {

		return Self {
			verts: verts.to_vec(),
			d_verts: verts.to_vec(),
			col: HashMap::new(),
			filter: comps![],
		};

	}

}

