// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::ecs::*;
use crate::comps::*;

pub struct PetalFollowSys;

impl System for PetalFollowSys {

	fn update(&mut self, pool: &mut Pool) {

		for id in pool.filter(&filter![Petal, Trans]) {

			let e = pool.get(id).unwrap();
			let petal = e.get::<Petal>();
			let mut trans = e.get::<Trans>();

			if let Some(flower) = pool.get(petal.flower) {

				let f_trans = flower.get::<Trans>();
				let ang = petal.index as f32 * 90f32;

				trans.pos = f_trans.pos + Vec2::from_angle((ang - 90.0).to_radians()) * 1.2;
				trans.rot = f_trans.rot + ang.to_radians();
				pool.get_mut(id).unwrap().set::<Trans>(trans);

			}

		}

	}

}







