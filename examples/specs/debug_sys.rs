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

			g2d::push();
			g2d::reset();
			g2d::line_width(2);
			g2d::color(color!(0, 1, 1, 1));
			g2d::poly(&b.d_verts);
			g2d::pop();

		}

	}

}


