// wengwengweng

use std::collections::HashMap;

use dirty::*;
use dirty::addons::res;
use dirty::addons::ecs::*;

pub struct Sprite {

	pub frame: usize,
	pub framelist: Vec<Rect>,
	pub name: String,
	pub origin: Vec2,
	pub anims: HashMap<String, res::Anim>,
	pub current_anim: Option<res::Anim>,
	pub speed: f32,
	pub timer: f32,

}

impl Sprite {

	pub fn new(name: &str) -> Self {

		let data = res::sprite(name);
		let frames = data.frames.clone();

		return Self {

			framelist: frames,
			frame: 0,
			name: name.to_owned(),
			origin: vec2!(0.5),
			anims: data.anims.clone(),
			current_anim: None,
			speed: 0.1,
			timer: 0.0,

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

	pub fn tex(&self) -> &gfx::Texture {
		return &res::sprite(&self.name).tex;
	}

	pub fn offset(&self) -> Vec2 {
		return vec2!(self.width(), self.height()) * self.origin * -1
	}

	pub fn width(&self) -> f32 {
		return self.tex().width() as f32 * self.framelist[self.frame].w;
	}

	pub fn height(&self) -> f32 {
		return self.tex().height() as f32 * self.framelist[self.frame].h;
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

impl Comp for Sprite {}

