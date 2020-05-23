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

struct SourceCtx {
	src: Arc<Mutex<dyn Source + Send>>,
	ctrl: Option<Arc<Mutex<Control>>>,
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
		self.sources.push(SourceCtx {
			src: src,
			ctrl: None,
			done: false,
		});
	}
	pub fn add_with_ctrl(&mut self, src: Arc<Mutex<dyn Source + Send>>, ctrl: Arc<Mutex<Control>>) {
		self.sources.push(SourceCtx {
			src: src,
			ctrl: Some(ctrl),
			done: false,
		});
	}
}

impl Iterator for Mixer {

	type Item = f32;

	fn next(&mut self) -> Option<Self::Item> {

		self.sources.retain(|ctx| {
			return !ctx.done;
		});

		return Some(self.sources.iter_mut().fold(0.0, |n, ctx| {

			if let Ok(mut src) = ctx.src.lock() {

				let mut paused = false;
				let mut volume = 1.0;

				if let Some(ctrl) = &ctx.ctrl {
					if let Ok(ctrl) = ctrl.lock() {
						paused = ctrl.paused;
						volume = ctrl.volume;
					}
				}

				if paused {
					return n;
				} else {

					if let Some(val) = src.next() {
						return n + val * volume;
					} else {
						ctx.done = true;
						return n;
					}
				}

			} else {
				return n;
			}

		}));

	}

}

