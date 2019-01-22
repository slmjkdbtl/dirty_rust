// wengwengweng

pub struct MoveSystem;

use dirty::*;
use dirty::addons::ecs::*;

use crate::trans::*;
use crate::vel::*;

impl System for MoveSystem {

	fn accept(&self) -> CompFilter {
		return comp_filter![Trans, Vel];
	}

	fn update(&self, e: &mut Entity) {

		let mut t = e.get::<Trans>();

		t.pos = t.pos + vec2!(3);
		e.set::<Trans>(t);

	}

}

