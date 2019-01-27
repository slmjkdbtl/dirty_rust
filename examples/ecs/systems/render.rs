// wengwengweng

use dirty::*;
use dirty::ecs::*;
use crate::comps::*;

pub struct RenderSys;

impl System for RenderSys {

	fn filter(&self) -> Filter {
		return filter![Sprite, Trans];
	}

	fn each(&mut self, e: &mut Entity) {

		let t = e.get::<Trans>();
		let s = e.get::<Sprite>();

		gfx::push();
		gfx::color(s.color);
		gfx::translate(t.pos);
		gfx::rotate(t.rot);
		gfx::translate(s.offset() * t.scale);
		gfx::scale(t.scale);

		if e.has::<Body>() {

			let mut body = e.get::<Body>();

			body.d_verts = gfx::multi_warp(&body.verts);
			e.set::<Body>(body);

		}

		gfx::draw(&s.tex, s.framelist[s.frame]);
		gfx::pop();

	}

}

