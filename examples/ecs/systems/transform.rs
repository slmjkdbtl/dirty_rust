// wengwengweng

use dirty::*;
use dirty::kit::*;
use crate::comps::*;

pub fn transform(pool: &mut Pool) {

	for id in pool.pick(&comps![Trans, Vel]) {

		let e = pool.get_mut(id).unwrap();
		let mut t = e.get::<Trans>();
		let v = e.get::<Vel>();

		t.pos = t.pos + v.pos * app::dt();
		t.rot = t.rot + v.rot * app::dt();
		e.set::<Trans>(t);

	}

}

