// wengwengweng

use std::collections::HashMap;

use dirty::*;
use dirty::math::*;
use dirty::addons::res;
use dirty::addons::ecs::*;

comp!(Sprite {

	tex: gfx::Texture,
	frame: usize,
	framelist: Vec<Rect>,
	origin: Vec2,
	anims: HashMap<String, res::Anim>,
	current_anim: Option<res::Anim>,
	speed: f32,
	color: Color,
	timer: f32,

});

impl Sprite {

	pub fn new(name: &str) -> Self {

		let data = res::sprite(name);
		let frames = data.frames.clone();

		return Self {

			framelist: frames,
			frame: 0,
			origin: vec2!(0.5),
			anims: data.anims.clone(),
			current_anim: None,
			speed: 0.1,
			timer: 0.0,
			tex: data.tex.clone(),
			color: color!(1),

		}

	}

	pub fn play(&mut self, name: &str) {

		if let Some(anim) = self.anims.get(name) {

			self.current_anim = Some(*anim);
			self.timer = 0.0;
			self.frame = anim.from;

		}
	}

	pub fn tick(&mut self) {

		if let Some(anim) = self.current_anim {
			match anim.dir {
				res::AnimDir::Forward => {
					if self.frame >= anim.to {
						self.frame = anim.from;
					} else {
						self.frame += 1;
					}
				}
				res::AnimDir::Reverse => {
					if self.frame <= anim.from {
						self.frame = anim.to;
					} else {
						self.frame -= 1;
					}
				}
				res::AnimDir::PingPong => {}
			}
		}

	}

	pub fn offset(&self) -> Vec2 {
		return vec2!(self.width(), self.height()) * self.origin * -1
	}

	pub fn width(&self) -> f32 {
		return self.tex.width() as f32 * self.framelist[self.frame].w;
	}

	pub fn height(&self) -> f32 {
		return self.tex.height() as f32 * self.framelist[self.frame].h;
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

