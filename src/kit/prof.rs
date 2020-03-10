// wengwengweng

use std::collections::HashMap;
use std::time::Instant;
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

}

