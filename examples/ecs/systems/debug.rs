// wengwengweng

use dirty::*;
use dirty::ecs::*;
use crate::comps::*;

pub fn debug(pool: &mut Pool) {

	for id in pool.pick(&filter![Body]) {

		if !app::debug() {
			return;
		}

		let e = pool.get(id).unwrap();
		let b = e.get::<Body>();

		g2d::push();
		g2d::reset();
		g2d::line_width(1);
		g2d::color(color!(0, 1, 1, 1));
		g2d::poly(&b.d_verts);
		g2d::pop();

	}

}

