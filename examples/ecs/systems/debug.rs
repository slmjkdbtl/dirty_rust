// wengwengweng

use sock::*;
use sock::ecs::*;
use crate::comps::*;

pub struct DebugSys;

impl System for DebugSys {

	fn filter(&self) -> Filter {
		return filter![Body];
	}

	fn each(&mut self, e: &mut Entity) {

		if !app::debug() {
			return;
		}

		let b = e.get::<Body>();

		g2d::push();
		g2d::reset();
		g2d::line_width(1);
		g2d::color(color!(0, 1, 1, 1));
		g2d::poly(&b.d_verts);
		g2d::pop();

	}

}


