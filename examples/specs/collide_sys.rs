// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

use crate::trans::*;
use crate::sprite::*;
use crate::body::*;

pub struct CollideSys;

impl System for CollideSys {

	fn filter(&self) -> CompFilter {
		return comp_filter![Body];
	}

	fn update(&self, e: &mut Entity) {

		let b = e.get::<Trans>();

	}

}


