// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

use crate::trans::*;
use crate::sprite::*;

pub struct RenderSystem;

impl System for RenderSystem {

	fn filter(&self) -> CompFilter {
		return comp_filter![Trans, Sprite];
	}

	fn update(&self, e: &mut Entity) {

		let t = e.get::<Trans>();
		let s = e.get::<Sprite>();

		gfx::draw(&s.tex, s.quad);

	}

}



