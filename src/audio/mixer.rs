// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;

use super::*;

pub(super) struct Control {
	pub paused: bool,
	pub volume: f32,
}

impl Control {
	pub fn new() -> Self {
		return Self {
			paused: false,
			volume: 1.0,
		};
	}
}

pub(super) struct Mixer {
	sources: Vec<(Arc<Mutex<dyn Source + Send>>, Option<Arc<Mutex<Control>>>)>,
}

impl Mixer {
	pub fn new() -> Self {
		return Self {
			sources: vec![],
		};
	}
	pub fn add(&mut self, src: Arc<Mutex<dyn Source + Send>>) {
		self.sources.push((src, None));
	}
	pub fn add_with_ctrl(&mut self, src: Arc<Mutex<dyn Source + Send>>, ctrl: Arc<Mutex<Control>>) {
		self.sources.push((src, Some(ctrl)));
	}
}

impl Iterator for Mixer {

	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {
		return Some(self.sources.iter_mut().fold(0.0, |n, (src, ctrl)| {
			if let Ok(mut src) = src.lock() {
				let mut paused = false;
				let mut volume = 1.0;
				if let Some(ctrl) = ctrl {
					if let Ok(ctrl) = ctrl.lock() {
						paused = ctrl.paused;
						volume = ctrl.volume;
					}
				}
				if paused {
					return n;
				} else {
					return n + src.next().unwrap_or(0.0) * volume;
				}
			} else {
				return n;
			}
		}));
	}

}

