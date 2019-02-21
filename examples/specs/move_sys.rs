// wengwengweng

use dirty::*;
use dirty::math::*;
use specs::*;

use crate::trans::*;
use crate::vel::*;

pub struct MoveSys;

impl<'a> System<'a> for MoveSys {

	type SystemData = (
		ReadStorage<'a, Vel>,
		WriteStorage<'a, Trans>
	);

	fn run(&mut self, (vel_storage, mut trans_storage): Self::SystemData) {
		for (vel, trans) in (&vel_storage, &mut trans_storage).join() {
			trans.pos = trans.pos + vel.pos;
		}
	}

}

