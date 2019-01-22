// wengwengweng

use dirty::*;
use dirty::addons::ecs::*;

comp!(Sprite {

	frame: usize,
	tex: gfx::Texture,
	quad: Rect,
	origin: Vec2,

});

impl Sprite {

	pub fn new(tex: gfx::Texture) -> Self {

		return Self {
			frame: 0,
			tex: tex,
			quad: rect!(0, 0, 0.25, 1),
			origin: vec2!(0.5),
		}

	}

	pub fn offset(&self) -> Vec2 {
		return vec2!(self.width(), self.height()) * self.origin * -1
	}

	pub fn width(&self) -> f32 {
		return self.tex.width() as f32 * self.quad.w;
	}

	pub fn height(&self) -> f32 {
		return self.tex.height() as f32 * self.quad.h;
	}

	pub fn get_verts(&self) -> Vec<Vec2> {

		return vec![

			vec2!(0, 0),
			vec2!(self.width(), 0),
			vec2!(self.width(), self.height()),
			vec2!(0, self.height()),

		];

	}

}

