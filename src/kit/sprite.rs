// wengwengweng

use std::collections::HashMap;
use std::time::Duration;

use crate::*;
use math::*;

#[derive(Clone)]
pub struct Sprite {
	frames: Vec<Quad>,
	cur_frame: usize,
	anims: HashMap<String, Anim>,
	cur_anim: Option<String>,
	timer: Duration,
	speed: Duration,
}

impl Sprite {

	pub fn new() -> Self {
		return Self {
			frames: vec![quad!(0, 0, 1, 1)],
			cur_frame: 0,
			anims: hmap![],
			cur_anim: None,
			timer: Duration::from_secs_f32(0.0),
			speed: Duration::from_secs_f32(0.1),
		};
	}

	pub fn load_ase(&mut self, json: &str) -> Result<()> {

		let data = ase::parse(json)?;

		self.frames = data.frames;

		for (name, anim) in data.anims {
			self.add_anim(&name, Anim {
				from: anim.from,
				to: anim.to,
				looping: true,
			});
		}

		return Ok(());

	}

	pub fn from_ase(json: &str) -> Result<Self> {

		let mut sprite = Self::new();

		sprite.load_ase(json)?;

		return Ok(sprite);

	}

	pub fn width(&self) -> f32 {
		return self.frames[self.cur_frame].w;
	}

	pub fn height(&self) -> f32 {
		return self.frames[self.cur_frame].h;
	}

	pub fn slice(&mut self, x: u8, y: u8) {

		let w = 1.0 / x as f32;
		let h = 1.0 / y as f32;

		self.frames.clear();

		for i in 0..x as usize {
			for j in 0..y as usize {
				self.frames.push(quad!(i as f32 * w, j as f32 * h, w, h));
			}
		}

	}

	pub fn add_anim(&mut self, name: &str, anim: Anim) {
		self.anims.insert(name.to_owned(), anim);
	}

	pub fn next(&mut self) {
		if self.cur_frame < self.frames.len() - 1 {
			self.cur_frame += 1;
		}
	}

	pub fn prev(&mut self) {
		if self.cur_frame > 0 {
			self.cur_frame -= 1;
		}
	}

	pub fn set_speed(&mut self, s: Duration) {
		self.speed = s;
	}

	pub fn play(&mut self, name: &str) {

		self.cur_anim = Some(String::from(name));

		if let Some(anim) = self.anims.get(name) {
			self.cur_frame = anim.from;
		}

	}

	pub fn update(&mut self, dt: Duration) {

		let anim = match &self.cur_anim {
			Some(cur_anim) => {
				match self.anims.get(cur_anim) {
					Some(anim) => anim,
					None => return
				}
			},
			None => return
		};

		self.timer += dt;

		if self.timer >= self.speed {
			if anim.from < anim.to {
				if self.cur_frame >= anim.to {
					if anim.looping {
						self.cur_frame = anim.from;
					} else {
						self.cur_anim = None;
					}
				} else {
					self.cur_frame += 1;
				}
			}
			self.timer = Duration::from_secs_f32(0.0);
		}

	}

	pub fn frame(&self) -> Quad {
		return self.frames[self.cur_frame];
	}

}

#[derive(Clone, Copy, Debug)]
pub struct Anim {
	pub from: usize,
	pub to: usize,
	pub looping: bool,
}

