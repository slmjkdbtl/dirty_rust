// wengwengweng

use dirty::*;
use dirty::math::*;
use specs::*;

use crate::body::*;

pub struct DebugSys;

impl<'a> System<'a> for DebugSys {

	type SystemData = (
		ReadStorage<'a, Body>
	);

	fn run(&mut self, (body_storage): Self::SystemData) {

		if !app::debug() {
			return;
		}

		for (b) in (&body_storage).join() {

			gfx::push();
			gfx::reset();
			gfx::line_width(2);
			gfx::color(color!(0, 1, 1, 1));
			gfx::poly(&b.d_verts);
			gfx::pop();

		}

	}

}


