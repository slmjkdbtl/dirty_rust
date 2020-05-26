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
	size: usize,
	strength: f32,
}

impl Delay {
	pub fn new(s: usize, d: f32) -> Self {
		return Self {
			buffer: VecDeque::with_capacity(s),
			size: s,
			strength: d,
		};
	}
}

impl Default for Delay {
	fn default() -> Self {
		return Self::new(0, 0.0);
	}
}

impl Effect for Delay {

	fn process(&mut self, f: Frame) -> Frame {

		self.buffer.push_back(f * self.strength);

		if self.buffer.len() > self.size - 1 {
			if let Some(ff) = self.buffer.pop_front() {
				return f + ff;
			}
		}

		return f;

	}

	fn leftover(&mut self) -> Option<Frame> {
		return self.buffer.pop_front();
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

	pub fn set_delay(&self, s: usize, f: f32) {
		if let Ok(mut delay) = self.delay.lock() {
			*delay = Delay::new(s, f);
		}
	}

}

