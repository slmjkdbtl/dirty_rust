// wengwengweng

use dirty::*;
use specs::*;

use crate::trans::*;
use crate::sprite::*;
use crate::body::*;

pub struct RenderSys;

impl<'a> System<'a> for RenderSys {

	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Trans>,
		ReadStorage<'a, Sprite>,
		WriteStorage<'a, Body>,
	);

	fn run(&mut self, (ent, trans, sprite, mut body): Self::SystemData) {

		for (e, t, s) in (&ent, &trans, &sprite).join() {

			gfx::push();
			gfx::translate(t.pos);
			gfx::rotate(t.rot);
			gfx::translate(s.offset());
			gfx::scale(t.scale);

			if let Some(body) = body.get_mut(e) {
				body.d_verts = gfx::multi_warp(&body.verts);
			}

			gfx::draw(&s.tex(), s.quad);
			gfx::pop();

		}

	}

}

