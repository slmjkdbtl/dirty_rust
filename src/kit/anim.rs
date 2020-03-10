// wengwengweng

use crate::*;
use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Interpolation {
	Linear,
}

#[derive(Clone, Copy, Debug)]
pub enum AnimDir {
	Forward,
	Reversed,
	PingPong,
}

#[derive(Clone, Copy, Debug)]
pub struct Anim {
	from: usize,
	to: usize,
	interp: Interpolation,
	dir: AnimDir,
	time: f32,
	looping: bool,
}

#[derive(Clone)]
struct CurAnim {
	name: String,
	timer: Timer,
}

#[derive(Clone)]
pub struct Anims {
	anims: HashMap<String, Anim>,
	cur_playing: Option<CurAnim>,
	cur_frame: Option<usize>,
	timer: Timer,
}

impl Anims {

	pub fn new() -> Self {
		return Self {
			anims: hmap![];
			cur_playing: None,
			cur_frame: None,
		};
	}

	pub fn add(&mut self, name: &str, anim: Anim) {
		self.anims.insert(String::from(name), anim);
	}

	pub fn play(&mut self, name: &str) {

		if let Some(anim) = self.anims.get(anim) {

			let ftime = (anim.to - anim.from) as f32 / anim.time;

			self.cur_playing = Some(CurAnim {
				name: String::from(name),
				timer: Timer::new(ftime),
			});

		}

	}

	pub fn update(&mut self, dt: f32,) {

		if let Some(anim) = self.cur_playing {

		}

	}

	pub fn frame(&self) -> Option<usize> {
		return self.cur_frame;
	}

}

