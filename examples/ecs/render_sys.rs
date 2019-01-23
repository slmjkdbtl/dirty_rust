// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

use crate::trans::*;
use crate::sprite::*;
use crate::body::*;

pub struct RenderSys;

impl System for RenderSys {

	fn filter(&self) -> CompFilter {
		return comp_filter![Trans, Sprite];
	}

	fn update(&self, e: &mut Entity) {

		let t = e.get::<Trans>();
		let s = e.get::<Sprite>();

		gfx::push();
		gfx::translate(t.pos);
		gfx::rotate(t.rot);
		gfx::translate(s.offset());
		gfx::scale(t.scale);

		if e.has::<Body>() {

			let mut body = e.get::<Body>();

			body.d_verts = gfx::multi_warp(&body.verts);
			e.set::<Body>(body);

		}

		gfx::draw(&s.tex, s.quad);
		gfx::pop();

	}

}

