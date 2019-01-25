// wengwengweng

use dirty::*;
use specs::*;

use crate::trans::*;
use crate::petal::*;

pub struct PetalFollowSys;

impl<'a> System<'a> for PetalFollowSys {

	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Petal>,
		WriteStorage<'a, Trans>,
	);

	fn run(&mut self, (entity_storage, petal_storage, mut trans_storage): Self::SystemData) {

		for (e, petal) in (&entity_storage, &petal_storage).join() {

			let f_entity = entity_storage.entity(petal.flower.id());

			if trans_storage.get(e).is_some() && trans_storage.get(f_entity).is_some() {

				let f_trans = trans_storage
					.get(f_entity)
					.expect("oh no").clone();

				let mut trans = trans_storage
					.get_mut(e)
					.expect("oh no");

				let ang = petal.index_as_num() as f32 * 90f32;

				trans.pos = f_trans.pos + Vec2::from_angle((ang - 90.0).to_radians()) * 1.2;
				trans.rot = f_trans.rot + ang.to_radians();

			}

		}

	}

}

