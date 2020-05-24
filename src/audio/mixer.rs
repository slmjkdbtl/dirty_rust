// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;

use super::*;

// TODO: deal with sample rate

// TODO: a better way to deal with control? a plugin system?
#[derive(Clone, Copy)]
pub(super) struct Control {
	pub paused: bool,
	pub volume: f32,
	pub pan: f32,
}

impl Default for Control {
	fn default() -> Self {
		return Self {
			paused: false,
			volume: 1.0,
			pan: 0.0,
		};
	}
}

struct SourceCtx {
	src: Arc<Mutex<dyn Source + Send>>,
	ctrl: Arc<Mutex<Control>>,
	done: bool,
}

pub(super) struct Mixer {
	sources: Vec<SourceCtx>,
}

impl Mixer {
	pub fn new() -> Self {
		return Self {
			sources: vec![],
		};
	}
	pub fn add(&mut self, src: Arc<Mutex<dyn Source + Send>>) {
		self.add_with_ctrl(src, Arc::new(Mutex::new(Control::default())));
	}
	pub fn add_with_ctrl(&mut self, src: Arc<Mutex<dyn Source + Send>>, ctrl: Arc<Mutex<Control>>) {
		self.sources.push(SourceCtx {
			src: src,
			ctrl: ctrl,
			done: false,
		});
	}
}

impl Iterator for Mixer {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		let sample = self.sources.iter_mut().fold((0.0, 0.0), |n, ctx| {

			let (left, right) = n;

			let mut src = match ctx.src.lock() {
				Ok(src) => src,
				Err(_) => return n,
			};

			let ctrl = match ctx.ctrl.lock() {
				Ok(ctrl) => *ctrl,
				Err(_) => Control::default(),
			};

			if ctrl.paused {

				return n;

			} else {

				if let Some((left_sample, right_sample)) = src.next() {

					return (
						left + left_sample * ctrl.volume * ctrl.pan.map(1.0, -1.0, 0.0, 2.0),
						right + right_sample * ctrl.volume * ctrl.pan.map(-1.0, 1.0, 0.0, 2.0),
					);

				} else {

					ctx.done = true;

					return n;

				}

			}

		});

		self.sources.retain(|ctx| {
			return !ctx.done;
		});

		return Some(sample);

	}

}

