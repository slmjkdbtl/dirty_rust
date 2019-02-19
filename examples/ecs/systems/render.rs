// wengwengweng

use dirty::*;
use dirty::ecs::*;
use crate::comps::*;
use crate::resources::*;

pub fn render(pool: &mut Pool) {

	let cam = pool.get_res::<Camera>().unwrap();

	g2d::push();
	g2d::translate(cam.pos);
	g2d::scale(cam.scale);

	for id in pool.pick(&filter![Sprite, Trans]) {

		let e = pool.get_mut(id).unwrap();
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

	g2d::pop();

}

