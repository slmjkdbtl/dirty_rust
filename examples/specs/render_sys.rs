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

	fn run(&mut self, (entity_storage, trans_storage, sprite_storage, mut body_storage): Self::SystemData) {

		for (e, trans, sprite) in (&entity_storage, &trans_storage, &sprite_storage).join() {

			gfx::push();
			gfx::color(sprite.color);
			gfx::translate(trans.pos);
			gfx::rotate(trans.rot);
			gfx::translate(sprite.offset() * trans.scale);
			gfx::scale(trans.scale);

			match sprite.flip {
				Flip::X => gfx::scale(vec2!(-1, 1)),
				Flip::Y => gfx::scale(vec2!(1, -1)),
				Flip::XY => gfx::scale(vec2!(-1, -1)),
				_ => {},
			}

			if let Some(body) = body_storage.get_mut(e) {
				body.d_verts = gfx::multi_warp(&body.verts);
			}

			gfx::draw(&sprite.tex, sprite.framelist[sprite.frame]);
			gfx::pop();

		}

	}

}

