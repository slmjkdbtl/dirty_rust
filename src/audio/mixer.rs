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
	cur_channel: Channel,
}

impl Mixer {
	pub fn new() -> Self {
		return Self {
			sources: vec![],
			cur_channel: Channel::Left,
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

	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {

		let cur_channel = self.cur_channel;

		let sample = self.sources.iter_mut().fold(0.0, |n, ctx| {

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

				if let Some(val) = src.next() {

					let volume = ctrl.volume * match cur_channel {
						Channel::Left => ctrl.pan.map(1.0, -1.0, 0.0, 2.0),
						Channel::Right => ctrl.pan.map(-1.0, 1.0, 0.0, 2.0),
					};

					return n + val * volume;

				} else {

					ctx.done = true;

					return n;

				}

			}

		});

		self.cur_channel = match self.cur_channel {
			Channel::Left => Channel::Right,
			Channel::Right => Channel::Left,
		};

		self.sources.retain(|ctx| {
			return !ctx.done;
		});

		return Some(sample);

	}

}

