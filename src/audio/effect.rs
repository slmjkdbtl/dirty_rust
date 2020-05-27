// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
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
			volume: v.max(0.0).min(1.0),
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
			pan: p.max(-1.0).min(1.0),
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
pub struct Overdrive {
	strength: f32,
}

impl Overdrive {
	pub fn new(s: f32) -> Self {
		return Self {
			strength: s.max(0.0).min(1.0),
		};
	}
}

impl Default for Overdrive {
	fn default() -> Self {
		return Self::new(0.0);
	}
}

impl Effect for Overdrive {

	fn process(&mut self, f: Frame) -> Frame {

		// https://rickyhan.com/jekyll/update/2018/02/06/rust-guitar-pedal-effects-dsp.html

		if self.strength == 0.0 {
			return f;
		}

		let a = f.left.abs();
		let b = f.right.abs();

		let l = if a >= 0.0 && a < 0.33 {
			a * 2.0
		} else if a >= 0.33 && a < 0.66 {
			let t = 2.0 - 3.0 * a;
			(3.0 - t * t) / 3.0
		} else {
			a
		};

		let r = if b >= 0.0 && b < 0.33 {
			b * 2.0
		} else if b >= 0.33 && b < 0.66 {
			let t = 2.0 - 3.0 * b;
			(3.0 - t * t) / 3.0
		} else {
			b
		};

		let ff = Frame::new(l, r);

		return f + (ff - f) * self.strength;

	}

}

// TODO
#[derive(Clone, Debug)]
pub struct Reverb {
}

impl Reverb {
	pub fn new(d: f32) -> Self {
		return Self {
		};
	}
}

impl Default for Reverb {
	fn default() -> Self {
		return Self::new(0.0);
	}
}

impl Effect for Reverb {

	fn process(&mut self, f: Frame) -> Frame {
		return f;
	}

}

#[derive(Clone, Debug)]
pub struct Delay {
	buffer: VecDeque<Frame>,
	len: usize,
	cycles: usize,
	decay: f32,
}

impl Delay {

	pub fn new(duration: Duration, cycles: usize, decay: f32) -> Self {

		let len = (duration.as_secs_f32() * SAMPLE_RATE as f32) as usize;
		let mut buffer = VecDeque::with_capacity(len * cycles);

		for _ in 0..len * cycles {
			buffer.push_back(Frame::new(0.0, 0.0));
		}

		return Self {
			buffer: buffer,
			len: len,
			cycles: cycles,
			decay: decay,
		};

	}

}

impl Default for Delay {
	fn default() -> Self {
		return Self::new(Duration::from_secs_f32(0.0), 0, 0.0);
	}
}

impl Effect for Delay {

	fn process(&mut self, f: Frame) -> Frame {

		if self.len == 0 || self.cycles == 0 || self.decay == 0.0 {
			return f;
		}

		let of = (0..self.cycles).fold(f, |frame_acc, i| {
			return frame_acc + self.buffer
				.get(i * self.len)
				.map(|sample| *sample * self.decay.powf((self.cycles - i) as f32))
				.unwrap_or_default()
				;
		});

		self.buffer.push_back(f);
		self.buffer.pop_front();

		return of;

	}

	fn leftover(&mut self) -> Option<Frame> {

		if self.buffer.is_empty() {
			return None;
		}

		let of = (0..self.cycles).fold(Frame::default(), |frame_acc, i| {
			return frame_acc + self.buffer
				.get(i * self.len)
				.map(|sample| *sample * self.decay.powf((self.cycles - i) as f32))
				.unwrap_or_default()
				;
		});

		self.buffer.pop_front();

		return Some(of);

	}

}

#[derive(Clone)]
pub(super) struct BasicEffectChain {
	pan: Arc<Mutex<Pan>>,
	volume: Arc<Mutex<Volume>>,
	delay: Arc<Mutex<Delay>>,
	overdrive: Arc<Mutex<Overdrive>>,
	reverb: Arc<Mutex<Reverb>>,
}

impl BasicEffectChain {

	pub fn new() -> Self {
		return Self {
			pan: Arc::new(Mutex::new(Pan::default())),
			volume: Arc::new(Mutex::new(Volume::default())),
			delay: Arc::new(Mutex::new(Delay::default())),
			overdrive: Arc::new(Mutex::new(Overdrive::default())),
			reverb: Arc::new(Mutex::new(Reverb::default())),
		};
	}

	pub fn chain(&self) -> Vec<Arc<Mutex<dyn Effect + Send>>> {
		return vec![
			self.overdrive.clone(),
			self.delay.clone(),
			self.reverb.clone(),
			self.pan.clone(),
			self.volume.clone(),
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

	pub fn set_overdrive(&self, s: f32) {
		if let Ok(mut overdrive) = self.overdrive.lock() {
			*overdrive = Overdrive::new(s);
		}
	}

	pub fn set_reverb(&self, d: f32) {
		if let Ok(mut reverb) = self.reverb.lock() {
			*reverb = Reverb::new(d);
		}
	}

	pub fn set_delay(&self, len: Duration, cycles: usize, d: f32) {
		if let Ok(mut delay) = self.delay.lock() {
			*delay = Delay::new(len, cycles, d);
		}
	}

}

