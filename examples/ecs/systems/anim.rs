// wengwengweng

use dirty::*;
use dirty::ecs::*;
use crate::comps::*;

pub struct AnimSys;

impl System for AnimSys {

	fn filter(&self) -> Filter {
		return filter![Sprite];
	}

	fn each(&mut self, e: &mut Entity) {

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




