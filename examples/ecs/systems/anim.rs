// wengwengweng

use dirty::*;
use dirty::ecs::*;
use crate::comps::*;

pub fn anim(pool: &mut Pool) {

	for id in pool.pick(&filter![Sprite]) {

		let e = pool.get_mut(id).unwrap();
		let mut s = e.get::<Sprite>();

		if s.current_anim.is_some() {

			if s.timer >= s.speed {
				s.timer = 0.0;
				s.tick();
			} else {
				s.timer += app::dt();
			}

			e.set::<Sprite>(s);

		}

	}

}

