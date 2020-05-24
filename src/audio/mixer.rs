// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;

use super::*;

// TODO: deal with sample rate

struct SourceCtx {
	src: Arc<Mutex<dyn Source + Send>>,
	paused: Arc<Mutex<bool>>,
	effects: Vec<Arc<Mutex<dyn Effect + Send>>>,
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
		self.add_ex(src, vec![]);
	}
	pub fn add_ex(&mut self, src: Arc<Mutex<dyn Source + Send>>, effects: Vec<Arc<Mutex<dyn Effect + Send>>>) {
		self.sources.push(SourceCtx {
			src: src,
			paused: Arc::new(Mutex::new(false)),
			effects: effects,
			done: false,
		});
	}
	pub fn add_ex_paused(&mut self, src: Arc<Mutex<dyn Source + Send>>, effects: Vec<Arc<Mutex<dyn Effect + Send>>>) -> Arc<Mutex<bool>> {
		let paused = Arc::new(Mutex::new(true));
		self.sources.push(SourceCtx {
			src: src,
			paused: Arc::clone(&paused),
			effects: effects,
			done: false,
		});
		return paused;
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

			if ctx.paused.lock().map(|b| *b).unwrap_or(false) {

				return n;

			} else {

				if let Some(mut frame) = src.next() {

					for e in &ctx.effects {
						if let Ok(mut e) = e.lock() {
							frame = e.frame(frame);
						}
					}

					return (
						left + frame.0,
						right + frame.1,
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

