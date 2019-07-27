// wengwengweng

//! Simple Threading Utilities

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

pub struct Pool {
	// ...
}

impl Pool {

	pub fn new(num: usize) -> Self {
		return Self {};
	}

	pub fn exec<T: Send + Clone + 'static, F: FnOnce() -> T + Send + 'static>(&mut self, f: F) -> Task<T> {

		let (tx, rx) = mpsc::channel();

		thread::spawn(move || {
			tx.send(f());
		});

		return Task::new(rx);

	}

}

pub fn exec<T: Send + Clone + 'static, F: FnOnce() -> T + Send + 'static>(f: F) -> Task<T> {

	let (tx, rx) = mpsc::channel();

	let t = thread::spawn(move || {
		tx.send(f());
	});

	t.join();

	return Task::new(rx);

}

pub struct Task<T> {
	rx: Receiver<T>,
}

impl<T> Task<T> {

	pub fn new(rx: Receiver<T>) -> Self {
		return Self {
			rx: rx,
		};
	}

	pub fn poll(&mut self) -> Option<T> {

		if let Ok(data) = self.rx.try_recv() {
			return Some(data);
		} else {
			return None;
		}

	}

}

