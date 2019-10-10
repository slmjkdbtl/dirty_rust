// wengwengweng

//! Simple Threading Utilities

use std::collections::VecDeque;

use std::sync::mpsc;
use std::thread;

pub struct TaskPool<T: Send + 'static> {
	queue: VecDeque<Task<T>>,
	active: Vec<Task<T>>,
	max: u32,
}

impl<T: Send + 'static> TaskPool<T> {

	pub fn new(max: u32) -> Self {
		return Self {
			queue: VecDeque::new(),
			active: vec![],
			max: max,
		};
	}

	pub fn exec(&mut self, f: impl FnOnce() -> T + Send + 'static) {
		self.queue.push_back(Task::new(f));
	}

	pub fn poll(&mut self) -> Vec<T> {

		if self.active.len() < self.max as usize {
			if let Some(mut task) = self.queue.pop_front() {
				task.start();
				self.active.push(task);
			}
		}

		let mut basket = vec![];

		for task in &mut self.active {
			if let Some(data) = task.poll() {
				basket.push(data);
			}
		}

		self.active.retain(|t| !t.done());

		return basket;

	}

}

pub struct Task<T: Send + 'static> {
	rx: Option<mpsc::Receiver<T>>,
	action: Option<Box<dyn FnOnce() -> T + Send + 'static>>,
	data: Option<T>,
	done: bool,
}

impl<T: Send + 'static> Task<T> {

	pub fn new(f: impl FnOnce() -> T + Send + 'static) -> Self {
		return Self {
			action: Some(Box::new(f)),
			data: None,
			done: false,
			rx: None,
		};
	}

	pub fn exec(f: impl FnOnce() -> T + Send + 'static) -> Self {

		let mut task = Self::new(f);

		task.start();

		return task;

	}

	// TODO: return error if no action
	pub fn start(&mut self) {

		if let Some(action) = self.action.take() {

			let (tx, rx) = mpsc::channel();

			// TODO: deal with error inside thread::spawn
			thread::spawn(move || {
				tx.send((action)()).expect("thread failure");
			});

			self.rx = Some(rx);

		}

	}

	pub fn started(&self) -> bool {
		return self.rx.is_some();
	}

	pub fn done(&self) -> bool {
		return self.done;
	}

	// TODO: return error if no rx
	pub fn block(&mut self) -> Option<T> {

		if let Some(rx) = &self.rx {
			self.done = true;
			return rx.recv().ok();
		} else {
			return None;
		}

	}

	// TODO: return error if no rx
	pub fn poll(&mut self) -> Option<T> {

		let rx = match &self.rx {
			Some(rx) => rx,
			None => return None,
		};

		if self.done {
			return None;
		}

		if let Ok(data) = rx.try_recv() {
			self.data = Some(data);
		}

		if self.data.is_some() {
			self.done = true;
			return self.data.take();
		} else {
			return None;
		}

	}

}

