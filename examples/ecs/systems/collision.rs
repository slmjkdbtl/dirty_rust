// wengwengweng

use dirty::*;
use dirty::kit::*;
use crate::comps::*;

pub fn collision(pool: &mut Pool) {

	let bodies = pool.pick(&comps![Body]);

	for i in 0..bodies.len() {

		let id1 = bodies[i];
		let e1 = pool.get(id1).unwrap();
		let mut b1 = e1.get::<Body>();

		b1.col.clear();

		for j in i..bodies.len() {

			let id2 = bodies[j];
			let e2 = pool.get(id2).unwrap();
			let mut b2 = e2.get::<Body>();
			let (collided, delta) = col::sat(&b1.d_verts, &b2.d_verts);

			if collided {

				b1.col.insert(id2, delta);
				b2.col.insert(id1, delta * -1);
				pool.update::<Body>(id2, b2);

			}

		}

		pool.update::<Body>(id1, b1);

	}

}

