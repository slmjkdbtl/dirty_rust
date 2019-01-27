// wengwengweng

use dirty::*;
use dirty::ecs::*;
use crate::comps::*;

pub struct TransformSys;

impl System for TransformSys {

	fn filter(&self) -> Filter {
		return filter![Trans, Vel];
	}

	fn each(&mut self, e: &mut Entity) {

		let mut t = e.get::<Trans>();
		let v = e.get::<Vel>();

		t.pos = t.pos + v.pos * app::dt();
		t.rot = t.rot + v.rot * app::dt();
		e.set::<Trans>(t);

	}

}

