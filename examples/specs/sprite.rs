// wengwengweng

use dirty::*;
use dirty::addons::res;
use specs::*;
use specs_derive::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {

	pub frame: usize,
	pub name: String,
	pub quad: Rect,
	pub origin: Vec2,

}

impl Sprite {

	pub fn new(name: &str) -> Self {

		return Self {
			frame: 0,
			name: name.to_owned(),
			quad: rect!(0, 0, 0.25, 1),
			origin: vec2!(0.5),
		}

	}

	pub fn tex(&self) -> &gfx::Texture {
		return &res::sprite(&self.name).tex;
	}

	pub fn offset(&self) -> Vec2 {
		return vec2!(self.width(), self.height()) * self.origin * -1
	}

	pub fn width(&self) -> f32 {
		return self.tex().width() as f32 * self.quad.w;
	}

	pub fn height(&self) -> f32 {
		return self.tex().height() as f32 * self.quad.h;
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

