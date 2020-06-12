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
}

impl Default for Control {
	fn default() -> Self {
		return Self {
			volume: 1.0,
			pan: Pan::new(1.0, 1.0),
			paused: false,
			detach: false,
			looping: false,
		};
	}
}

struct SourceCtx {
	src: Arc<Mutex<dyn Source + Send>>,
	control: Arc<Mutex<Control>>,
	effects: Vec<Arc<Mutex<dyn Effect + Send>>>,
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
			src: src,
			control: Arc::new(Mutex::new(Control::default())),
			effects: vec![],
		});

		self.last_id += 1;

		return Ok(id);

	}

	pub fn get_control(&self, id: &SourceID) -> Option<Arc<Mutex<Control>>> {
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

					for e in &ctx.effects {
						if let Ok(mut e) = e.lock() {
							frame = e.process(frame);
						}
					}

					return frame_acc + frame * ctrl.pan * ctrl.volume;

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

