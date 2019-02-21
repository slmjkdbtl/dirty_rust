// wengwengweng

use dirty::*;
use dirty::math::*;
use specs::*;

use crate::sprite::*;

pub struct AnimSys;

impl<'a> System<'a> for AnimSys {

	type SystemData = (
		WriteStorage<'a, Sprite>
	);

	fn run(&mut self, (mut sprite_storage): Self::SystemData) {

		for (s) in (&mut sprite_storage).join() {

			if let Some(anim) = s.current_anim {

				if s.timer >= s.speed {
					s.timer = 0.0;
					s.tick();
				} else {
					s.timer += app::dt();
				}

			}

		}

	}

}


