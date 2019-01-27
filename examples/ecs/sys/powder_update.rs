// wengwengweng

use dirty::*;
use dirty::math::*;
use dirty::ecs::*;
use crate::comps::*;

pub struct PowderUpdateSys;

impl System for PowderUpdateSys {

	fn update(&mut self, pool: &mut Pool) {

		for id in pool.filter(&filter![Powder, Sprite, Trans]) {

			let e = pool.get(id).unwrap();
			let p = e.get::<Powder>();

			if let Some(flower) = pool.get(p.flower) {

				let f = flower.get::<Flower>();
				let fv = flower.get::<Vel>();

				if f.active {

					let e = pool.get_mut(id).unwrap();
					let mut sprite = e.get::<Sprite>();
					let mut t = e.get::<Trans>();

					sprite.color = color!(rand!(2) as i32, rand!(2) as i32, rand!(2) as i32, 1);
					t.pos = t.pos + Vec2::from_angle(p.dir) * p.speed * app::dt() + fv.pos * app::dt();

					e.set::<Sprite>(sprite);
					e.set::<Trans>(t);

				}

			}

		}

	}

}



