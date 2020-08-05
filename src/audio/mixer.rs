// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

use super::*;

pub(super) type SourceID = usize;

pub(super) struct Control {
	pub volume: f32,
	pub pan: Pan,
	pub paused: bool,
	pub detach: bool,
	pub looping: bool,
	pub effects: Vec<Arc<Mutex<dyn Effect + Send>>>,
}

impl Default for Control {
	fn default() -> Self {
		return Self {
			volume: 1.0,
			pan: Pan::new(1.0, 1.0),
			paused: false,
			detach: false,
			looping: false,
			effects: vec![],
		};
	}
}

struct SourceCtx {
	src: Arc<Mutex<dyn Source + Send>>,
	control: Arc<Mutex<Control>>,
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

	pub fn add(&mut self, src: Arc<Mutex<dyn Source + Send>>) -> Arc<Mutex<Control>> {

		let id = self.last_id;
		let ctrl = Arc::new(Mutex::new(Control::default()));

		self.sources.insert(id, SourceCtx {
			src: src,
			control: ctrl.clone(),
		});

		self.last_id += 1;

		return ctrl;

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

		let mut detached = hset![];

		let sample = self.sources
			.iter_mut()
			.fold(Frame::new(0.0, 0.0), |frame_acc, (id, ctx)| {

				let ctrl = match ctx.control.lock() {
					Ok(ctrl) => ctrl,
					Err(_) => return frame_acc,
				};

				let mut src = match ctx.src.lock() {
					Ok(src) => src,
					Err(_) => return frame_acc,
				};

				if ctrl.detach {
					detached.insert(*id);
				}

				if ctrl.paused {

					return frame_acc;

				} else if let Some(mut frame) = src.next() {

					for e in &ctrl.effects {
						if let Ok(mut e) = e.lock() {
							frame = e.process(frame);
						}
					}

					return frame_acc + frame * ctrl.pan * ctrl.volume;

				} else {

					let mut has_leftover = false;
					let mut leftover_acc = Frame::default();

					for i in 0..ctrl.effects.len() {
						if let Ok(mut e) = ctrl.effects[i].lock() {
							if let Some(mut leftover) = e.leftover() {
								has_leftover = true;
								for j in (i + 1)..ctrl.effects.len() {
									if let Ok(mut e2) = ctrl.effects[j].lock() {
										leftover = e2.process(leftover);
									}
								}
								leftover_acc += leftover * ctrl.pan * ctrl.volume;
							}
						};
					}

					if has_leftover {
						return frame_acc + leftover_acc;
					} else {
						if ctrl.looping {
							if let Err(e) = src.seek_start() {
								elog!("{}", e);
							}
						} else {
							detached.insert(*id);
						}
					}

					return frame_acc;

				}

			});

		self.sources.retain(|id, ctx| {
			return !detached.contains(id);
		});

		return Some(sample);

	}

}

