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

	pub fn exec<T: Send + Clone + 'static, F: FnMut() -> T + Send + 'static>(&mut self, mut f: F) -> Task<T> {

		let (tx, rx) = mpsc::channel();

		thread::spawn(move || {
			tx.send(f());
		});

		return Task::new(rx);

	}

}

pub fn exec<T: Send + Clone + 'static, F: FnMut() -> T + Send + 'static>(mut f: F) -> Task<T> {

	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		tx.send(f());
	});

	return Task::new(rx);

}

pub struct Task<T> {
	rx: Receiver<T>,
	data: Option<T>,
}

impl<T: Clone> Task<T> {

	pub fn new(rx: Receiver<T>) -> Self {
		return Self {
			rx: rx,
			data: None,
		};
	}

	pub fn data(&self) -> Option<T> {
		return self.data.clone();
	}

	pub fn done(&self) -> bool {
		return self.data.is_some();
	}

	pub fn poll(&mut self) {

		if self.done() {
			return;
		}

		if let Ok(data) = self.rx.try_recv() {
			self.data = Some(data);
		}

	}

}

