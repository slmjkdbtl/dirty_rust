// wengwengweng

use std::collections::HashMap;
use std::hash::Hash;

pub struct State<T> {

	current: T,
	on_events: HashMap<T, Box<Fn(T)>>,
	leave_events: HashMap<T, Box<Fn(T)>>,
	during_events: HashMap<T, Box<Fn()>>,

}

impl<T: Hash + Eq + Copy> State<T> {

	pub fn new(current: T) -> Self {

		return Self {

			current: current,
			on_events: HashMap::new(),
			leave_events: HashMap::new(),
			during_events: HashMap::new(),

		};

	}

	pub fn set(&mut self, now: T) {

		if now == self.current {
			return;
		}

		let prev = self.current;

		if let Some(f) = self.leave_events.get(&self.current) {
			f(now);
		}

		self.current = now;

		if let Some(f) = self.on_events.get(&self.current) {
			f(prev);
		}

	}

	pub fn update(&self) {
		if let Some(f) = self.during_events.get(&self.current) {
			f();
		}
	}

	pub fn on<F: 'static + Fn(T)>(&mut self, trigger: T, f: F) {
		self.on_events.insert(trigger, Box::new(f));
	}

	pub fn leave<F: 'static + Fn(T)>(&mut self, trigger: T, f: F) {
		self.leave_events.insert(trigger, Box::new(f));
	}

	pub fn during<F: 'static + Fn()>(&mut self, trigger: T, f: F) {
		self.during_events.insert(trigger, Box::new(f));
	}

}

