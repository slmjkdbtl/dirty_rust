// wengwengweng

use super::*;

use std::collections::VecDeque;
use std::collections::HashMap;

const BUF_SIZE: usize = 128;

pub struct BasicSynth {
	notes: HashMap<Note, Voice>,
	volume: f32,
	buf: VecDeque<f32>,
	clock: f32,
	sample_rate: u32,
}

impl BasicSynth {

	pub fn new() -> Self {
		return BasicSynth {
			volume: 1.0,
			notes: hmap![],
			buf: VecDeque::with_capacity(BUF_SIZE),
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

impl Stream for BasicSynth {

	fn next(&mut self) -> Frame {

		let dt = 1.0 / SPEC.sample_rate as f32;

		self.clock += dt;

		let mut frame = 0.0;

		for n in self.notes.values_mut() {
			frame += n.voice(self.clock);
		}

		frame *= self.volume;

		for n in self.notes.values_mut() {
			n.tick(dt);
		}

		self.notes.retain(|_, n| !n.dead());

		if self.buf.len() >= self.buf.capacity() {
			self.buf.pop_front();
		}

		self.buf.push_back(frame);

		return Frame::new(frame, frame);

	}

}

