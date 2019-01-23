// wengwengweng

use dirty::*;
use specs::*;

use crate::body::*;

pub struct DebugSys;

impl<'a> System<'a> for DebugSys {

	type SystemData = (
		ReadStorage<'a, Body>
	);

	fn run(&mut self, (body): Self::SystemData) {

		for (b) in (&body).join() {

			gfx::push();
			gfx::line_width(2);
			gfx::color(color!(0, 1, 1, 1));
			gfx::poly(&b.d_verts);
			gfx::pop();

		}

	}

}


