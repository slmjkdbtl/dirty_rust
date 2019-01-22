// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

comp!(Sprite {

	frame: usize,
	tex: gfx::Texture,
	quad: Rect,

});

impl Sprite {
	pub fn new(tex: gfx::Texture) -> Self {
		return Self {
			frame: 0,
			tex: tex,
			quad: rect!(0, 0, 1, 1),
		}
	}
}

