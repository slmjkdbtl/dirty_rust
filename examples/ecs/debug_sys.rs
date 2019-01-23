// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

use crate::body::*;

pub struct DebugSys;

impl System for DebugSys {

	fn filter(&self) -> CompFilter {
		return comp_filter![Body];
	}

	fn update(&self, e: &mut Entity) {

		let body = e.get::<Body>();

		gfx::push();
		gfx::line_width(2);
		gfx::color(color!(0, 1, 1, 1));
		gfx::poly(&body.d_verts);
		gfx::pop();

	}

}

