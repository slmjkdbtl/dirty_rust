// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::collections::HashMap;

use super::*;

pub(super) type SourceID = usize;

pub(super) struct Control {
	paused: AtomicBool,
	remove: AtomicBool,
}

impl Control {
	pub fn remove(&self) {
		self.remove.store(true, Ordering::SeqCst);
	}
	pub fn removed(&self) -> bool {
		return self.remove.load(Ordering::SeqCst);
	}
	pub fn set_paused(&self, b: bool) {
		self.paused.store(b, Ordering::SeqCst);
	}
	pub fn paused(&self) -> bool {
		return self.paused.load(Ordering::SeqCst);
	}
}

impl Default for Control {
	fn default() -> Self {
		return Self {
			paused: AtomicBool::new(false),
			remove: AtomicBool::new(false),
		};
	}
}

struct SourceCtx {
	src: Arc<Mutex<dyn Source + Send>>,
	control: Arc<Control>,
	effects: Vec<Arc<Mutex<dyn Effect + Send>>>,
	done: bool,
}

pub(super) struct Mixer {
	sources: HashMap<SourceID, SourceCtx>,
	last_id: SourceID,
}

impl Mixer {

	pub fn new() -> Self {
		return Self {
			sources: hmap![],
			last_id: 0,
		};
	}

	pub fn add(&mut self, src: Arc<Mutex<dyn Source + Send>>) -> SourceID {

		let id = self.last_id;

		self.sources.insert(id, SourceCtx {
			src,
			control: Arc::new(Control::default()),
			effects: vec![],
			done: false,
		});

		self.last_id += 1;

		return id;

	}

	pub fn get_control(&self, id: &SourceID) -> Option<Arc<Control>> {
		return self.sources.get(&id).map(|ctx| {
			return ctx.control.clone();
		});
	}

	pub fn add_effect(&mut self, id: &SourceID, e: Arc<Mutex<dyn Effect + Send>>) {
		if let Some(ctx) = self.sources.get_mut(&id) {
			ctx.effects.push(e);
		}
	}

}

impl Iterator for Mixer {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		let sample = self.sources.iter_mut().fold(Frame::new(0.0, 0.0), |frame_acc, (id, ctx)| {

			let mut src = match ctx.src.lock() {
				Ok(src) => src,
				Err(_) => return frame_acc,
			};

			if ctx.control.paused() {

				return frame_acc;

			} else if let Some(mut frame) = src.next() {

				for e in &ctx.effects {
					if let Ok(mut e) = e.lock() {
						frame = e.process(frame);
					}
				}

				return Frame::new(
					frame_acc.left + frame.left,
					frame_acc.right + frame.right,
				);

			} else {

				ctx.done = true;

				return frame_acc;

			}

		});

		self.sources.retain(|_, ctx| {
			return !ctx.done || ctx.control.removed();
		});

		return Some(sample);

	}

}

