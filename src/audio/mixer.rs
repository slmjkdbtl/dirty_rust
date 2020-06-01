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
	detach: AtomicBool,
	looping: AtomicBool,
}

impl Control {
	pub fn detach(&self) {
		self.detach.store(true, Ordering::SeqCst);
	}
	fn detaching(&self) -> bool {
		return self.detach.load(Ordering::SeqCst);
	}
	pub fn set_paused(&self, b: bool) {
		self.paused.store(b, Ordering::SeqCst);
	}
	pub fn paused(&self) -> bool {
		return self.paused.load(Ordering::SeqCst);
	}
	pub fn set_looping(&self, l: bool) {
		self.looping.store(l, Ordering::SeqCst);
	}
	pub fn looping(&self) -> bool {
		return self.looping.load(Ordering::SeqCst);
	}
}

impl Default for Control {
	fn default() -> Self {
		return Self {
			paused: AtomicBool::new(false),
			detach: AtomicBool::new(false),
			looping: AtomicBool::new(false),
		};
	}
}

struct SourceCtx {
	src: Converter,
	control: Arc<Control>,
	effects: Vec<Arc<Mutex<dyn Effect + Send>>>,
	done: bool,
}

pub(super) struct Mixer {
	sources: HashMap<SourceID, SourceCtx>,
	last_id: SourceID,
	spec: Spec,
}

impl Mixer {

	pub fn new(spec: Spec) -> Self {
		return Self {
			sources: hmap![],
			last_id: 0,
			spec: spec,
		};
	}

	pub fn add(&mut self, src: Arc<Mutex<dyn Source + Send>>) -> Result<SourceID> {

		let id = self.last_id;

		self.sources.insert(id, SourceCtx {
			src: Converter::new(src, self.spec)?,
			control: Arc::new(Control::default()),
			effects: vec![],
			done: false,
		});

		self.last_id += 1;

		return Ok(id);

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

	pub fn count(&self) -> usize {
		return self.sources.len();
	}

}

impl Iterator for Mixer {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		if self.sources.is_empty() {
			return None;
		}

		let sample = self.sources.iter_mut().fold(Frame::new(0.0, 0.0), |frame_acc, (id, ctx)| {

			if ctx.control.paused() {

				return frame_acc;

			} else if let Some(mut frame) = ctx.src.next() {

				for e in &ctx.effects {
					if let Ok(mut e) = e.lock() {
						frame = e.process(frame);
					}
				}

				return frame_acc + frame;

			} else {

				let mut has_leftover = false;
				let mut leftover_acc = Frame::default();

				for i in 0..ctx.effects.len() {
					if let Ok(mut e) = ctx.effects[i].lock() {
						if let Some(mut leftover) = e.leftover() {
							has_leftover = true;
							for j in (i + 1)..ctx.effects.len() {
								if let Ok(mut e2) = ctx.effects[j].lock() {
									leftover = e2.process(leftover);
								}
							}
							leftover_acc = leftover_acc + leftover;
						}
					};
				}

				if has_leftover {
					return frame_acc + leftover_acc;
				} else {
					if ctx.control.looping() {
						if let Ok(mut src) = ctx.src.get_inner().lock() {
							if let Err(e) = src.seek_start() {
								elog!("{}", e);
							}
						}
					} else {
						ctx.done = true;
					}
				}

				return frame_acc;

			}

		});

		self.sources.retain(|_, ctx| {
			return !ctx.done || ctx.control.detaching();
		});

		return Some(sample);

	}

}

