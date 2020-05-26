// wengwengweng

use std::collections::VecDeque;
use super::*;

/// Chainable Audio Effect
pub trait Effect {
	fn process(&mut self, _: Frame) -> Frame;
	fn leftover(&mut self) -> Option<Frame> {
		return None;
	}
}

#[derive(Clone, Debug)]
pub struct Volume {
	volume: f32,
}

impl Volume {
	pub fn new(v: f32) -> Self {
		return Self {
			volume: v,
		};
	}
}

impl Default for Volume {
	fn default() -> Self {
		return Self::new(1.0);
	}
}

impl Effect for Volume {
	fn process(&mut self, f: Frame) -> Frame {
		return f * self.volume;
	}
}

#[derive(Clone, Debug)]
pub struct Pan {
	pan: f32,
}

impl Pan {
	pub fn new(p: f32) -> Self {
		return Self {
			pan: p,
		};
	}
}

impl Default for Pan {
	fn default() -> Self {
		return Self::new(0.0);
	}
}

impl Effect for Pan {
	fn process(&mut self, f: Frame) -> Frame {
		return Frame::new(
			f.left * self.pan.map(1.0, -1.0, 0.0, 2.0),
			f.right * self.pan.map(-1.0, 1.0, 0.0, 2.0),
		);
	}
}

#[derive(Clone, Debug)]
pub struct Delay {
	buffer: VecDeque<Frame>,
	len: usize,
	cycles: usize,
	decay: f32,
	filled: bool,
}

impl Delay {
	pub fn new(len: usize, c: usize, d: f32) -> Self {
		return Self {
			buffer: VecDeque::with_capacity(len * c),
			len: len,
			cycles: c,
			decay: d,
			filled: false,
		};
	}
}

impl Default for Delay {
	fn default() -> Self {
		return Self::new(0, 0, 0.0);
	}
}

impl Effect for Delay {

	fn process(&mut self, f: Frame) -> Frame {

		if self.len == 0 || self.cycles == 0 {
			return f;
		}

		let mut of = f;

		for i in 0..self.cycles {
			if self.buffer.len() as isize - (self.len * i) as isize >= 0 {
				if let Some(frame) = self.buffer.get(self.buffer.len() - (self.len * i)) {
					of = of + *frame * self.decay.powf(i as f32);
				}
			}
		}

		self.buffer.push_back(f);

		if self.buffer.len() > self.len * self.cycles {
			self.filled = true;
			self.buffer.pop_front();
		}

		return of;

	}

	fn leftover(&mut self) -> Option<Frame> {

		let mut has_left = false;
		let mut of = Frame::default();

		for i in 0..self.cycles {
			if self.buffer.len() as isize - (self.len * i) as isize >= 0 {
				if let Some(frame) = self.buffer.get(self.buffer.len() - (self.len * i)) {
					has_left = true;
					of = of + *frame * self.decay.powf(i as f32);
				}
			}
		}

		if !self.filled {
			if self.buffer.len() < self.len * self.cycles {
				has_left = true;
				self.buffer.push_back(Frame::default());
			} else {
				self.filled = true;
			}
		} else {
			self.buffer.pop_front();
		}

		if has_left {
			return Some(of);
		} else {
			return None;
		}

	}

}

use std::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub(super) struct BasicEffectChain {
	pan: Arc<Mutex<Pan>>,
	volume: Arc<Mutex<Volume>>,
	delay: Arc<Mutex<Delay>>,
}

impl BasicEffectChain {

	pub fn new() -> Self {
		return Self {
			pan: Arc::new(Mutex::new(Pan::default())),
			volume: Arc::new(Mutex::new(Volume::default())),
			delay: Arc::new(Mutex::new(Delay::default())),
		};
	}

	pub fn chain(&self) -> Vec<Arc<Mutex<dyn Effect + Send>>> {
		return vec![
			self.delay.clone(),
			self.pan.clone(),
			self.volume.clone()
		];
	}

	pub fn set_pan(&self, p: f32) {
		if let Ok(mut pan) = self.pan.lock() {
			*pan = Pan::new(p);
		}
	}

	pub fn set_volume(&self, v: f32) {
		if let Ok(mut volume) = self.volume.lock() {
			*volume = Volume::new(v);
		}
	}

	pub fn set_delay(&self, len: usize, cycles: usize, d: f32) {
		if let Ok(mut delay) = self.delay.lock() {
			*delay = Delay::new(len, cycles, d);
		}
	}

}

