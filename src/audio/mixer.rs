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

pub struct SampleRateConverter {
	src: Arc<Mutex<dyn Source + Send>>,
	target: u32,
	prev_frame: Option<Frame>,
	next_frame: Option<Frame>,
	pos: f32,
	frame_pos: usize,
}

impl SampleRateConverter {
	fn new(src: Arc<Mutex<dyn Source + Send>>, target: u32) -> Result<Self> {
		return Ok(Self {
			src: src,
			target: target,
			prev_frame: None,
			next_frame: None,
			pos: 0.0,
			frame_pos: 0,
		})
	}
	fn get_inner(&self) -> &Arc<Mutex<dyn Source + Send>> {
		return &self.src;
	}
}

impl Iterator for SampleRateConverter {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {

		let mut src = match self.src.lock() {
			Ok(src) => src,
			Err(_) => return None,
		};

		let sample_rate = src.sample_rate();

		if self.target == sample_rate {
			return src.next();
		}

		// TODO: bugged yo
		let speed = sample_rate as f32 / self.target as f32;

		self.pos += speed;

		if self.pos > self.frame_pos as f32 {
			let skip = self.pos as usize - self.frame_pos;
			for _ in 0..skip {
				src.next();
			}
			self.prev_frame = self.next_frame;
			self.next_frame = src.next();

			if self.next_frame.is_none() {
				return None;
			}

			self.frame_pos += skip + 1;
			let progress = self.pos - (self.frame_pos - 1) as f32;
			let prev = self.prev_frame.unwrap_or_default();
			let next = self.next_frame.unwrap_or_default();
			return Some(prev + (next - prev) * progress);
		} else {
			let prev = self.prev_frame.unwrap_or_default();
			let next = self.next_frame.unwrap_or_default();
			let progress = self.pos - (self.frame_pos - 1) as f32;
			return Some(prev + (next - prev) * progress);
		}

	}

}

struct SourceCtx {
	src: SampleRateConverter,
	control: Arc<Control>,
	effects: Vec<Arc<Mutex<dyn Effect + Send>>>,
	done: bool,
}

pub(super) struct Mixer {
	sources: HashMap<SourceID, SourceCtx>,
	last_id: SourceID,
	sample_rate: u32,
}

impl Mixer {

	pub fn new(sample_rate: u32) -> Self {
		return Self {
			sources: hmap![],
			last_id: 0,
			sample_rate,
		};
	}

	pub fn add(&mut self, src: Arc<Mutex<dyn Source + Send>>) -> Result<SourceID> {

		let id = self.last_id;

		self.sources.insert(id, SourceCtx {
			src: SampleRateConverter::new(src, self.sample_rate)?,
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

