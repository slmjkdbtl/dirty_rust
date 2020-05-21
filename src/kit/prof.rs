// wengwengweng

use instant::Instant;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Copy)]
enum EventState {
	InProgress(Instant),
	Completed(Duration),
}

#[derive(Clone)]
pub struct Profiler {
	events: HashMap<String, EventState>,
}

impl Profiler {

	pub fn new() -> Self {
		return Self {
			events: hmap![],
		};
	}

	pub fn event<R, F: FnOnce() -> R>(&mut self, name: &str, f: F) -> R {
		self.begin(name);
		let r = f();
		self.end(name);
		return r;
	}

	pub fn begin(&mut self, name: &str) {
		self.events
			.insert(String::from(name), EventState::InProgress(Instant::now()));
	}

	pub fn end(&mut self, name: &str) {
		if let Some(e) = self.events.get_mut(name) {
			if let EventState::InProgress(i) = e {
				*e = EventState::Completed(i.elapsed());
			}
		}
	}

	pub fn get(&self, name: &str) -> Option<Duration> {

		if let Some(e) = self.events.get(name) {
			if let EventState::Completed(d) = e {
				return Some(*d);
			}
		}

		return None;

	}

	pub fn list(&self) -> Vec<(&str, Duration)> {

		let mut list = vec![];

		for (name, state) in &self.events {
			if let EventState::Completed(time) = state {
				list.push((name.as_str(), *time));
			}
		}

		list.sort_by(|(_, t1), (_, t2)| {
			return std::cmp::Ord::cmp(t2, t1);
		});

		return list;

	}

}

