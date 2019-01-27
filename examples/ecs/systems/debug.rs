// wengwengweng

use dirty::*;
use dirty::ecs::*;
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

		gfx::push();
		gfx::reset();
		gfx::line_width(1);
		gfx::color(color!(0, 1, 1, 1));
		gfx::poly(&b.d_verts);
		gfx::pop();

	}

}


