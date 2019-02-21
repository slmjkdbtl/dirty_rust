// wengwengweng

use dirty::*;
use dirty::math::*;
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

	fn run(&mut self, (entity_storage, trans_storage, sprite_storage, mut body_storage): Self::SystemData) {

		for (e, trans, sprite) in (&entity_storage, &trans_storage, &sprite_storage).join() {

			g2d::push();
			g2d::color(sprite.color);
			g2d::translate(trans.pos);
			g2d::rotate(trans.rot);
			g2d::translate(sprite.offset() * trans.scale);
			g2d::scale(trans.scale);

			if let Some(body) = body_storage.get_mut(e) {
				body.d_verts = g2d::multi_warp(&body.verts);
			}

			g2d::draw(&sprite.tex, sprite.framelist[sprite.frame]);
			g2d::pop();

		}

	}

}

