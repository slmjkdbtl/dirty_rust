// wengwengweng

use super::*;

use std::collections::VecDeque;
use std::collections::HashMap;

pub struct BasicSynth {
	notes: HashMap<Note, Voice>,
	volume: f32,
	last_time: f32,
	buf: VecDeque<f32>,
	clock: f32,
	sample_rate: u32,
}

impl BasicSynth {

	pub fn new() -> Self {
		return BasicSynth {
			volume: 1.0,
			notes: hmap![],
			last_time: 0.0,
			buf: VecDeque::with_capacity(120),
			clock: 0.0,
			sample_rate: SPEC.sample_rate,
		};
	}

	pub fn buf(&self) -> &VecDeque<f32> {
		return &self.buf;
	}

	pub fn volume(&self) -> f32 {
		return self.volume;
	}

	pub fn set_volume(&mut self, v: f32) {
		self.volume = v.max(0.0).min(1.0);
	}

	pub fn play(&mut self, v: Voice) {
		self.notes.insert(v.note, v);
	}

	pub fn play_oneshot(&mut self, v: Voice) {

		let n = v.note;

		self.notes.insert(n, v);
		self.release(n);

	}

	pub fn release(&mut self, n: Note) {

		if let Some(n) = self.notes.get_mut(&n) {
			n.release();
		}

	}

}

impl Source for BasicSynth {
	fn sample_rate(&self) -> u32 {
		return self.sample_rate;
	}
}

impl Iterator for BasicSynth {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		self.clock = (self.clock + 1.0) % self.sample_rate as f32;

		let time = self.clock / self.sample_rate as f32;

		let dt = if time >= self.last_time {
			time - self.last_time
		} else {
			(1.0 + time) - self.last_time
		};

		self.last_time = time;

		let mut sound = 0.0;

		for n in self.notes.values_mut() {
			sound += n.voice(time);
		}

		sound *= self.volume;

		for n in self.notes.values_mut() {
			n.tick(dt);
		}

		self.notes.retain(|_, n| !n.dead());

		if self.buf.len() >= self.buf.capacity() {
			self.buf.pop_front();
		}

		self.buf.push_back(sound);

		return Some(Frame::new(sound, sound));

	}

}

