// wengwengweng

use sock::*;
use sock::ecs::*;
use crate::comps::*;

pub struct RenderSys;

impl System for RenderSys {

	fn filter(&self) -> Filter {
		return filter![Sprite, Trans];
	}

	fn each(&mut self, e: &mut Entity) {

		let t = e.get::<Trans>();
		let s = e.get::<Sprite>();

		g2d::push();
		g2d::color(s.color);
		g2d::translate(t.pos);
		g2d::rotate(t.rot);
		g2d::translate(s.offset() * t.scale);
		g2d::scale(t.scale);

		if e.has::<Body>() {

			let mut body = e.get::<Body>();

			body.d_verts = g2d::multi_warp(&body.verts);
			e.set::<Body>(body);

		}

		g2d::draw(&s.tex, s.framelist[s.frame]);
		g2d::pop();

	}

}

