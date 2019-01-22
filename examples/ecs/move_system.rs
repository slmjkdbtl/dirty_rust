// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

use crate::trans::*;
use crate::vel::*;

pub struct MoveSystem;

impl System for MoveSystem {

	fn filter(&self) -> CompFilter {
		return comp_filter![Trans, Vel];
	}

	fn update(&self, e: &mut Entity) {

		let mut t = e.get::<Trans>();
		let v = e.get::<Vel>();

		t.pos = t.pos + v.vel;
		t.rot = t.rot + app::dt();
		e.set::<Trans>(t);

	}

}

