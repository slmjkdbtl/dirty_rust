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

		return Task::from_rx(rx);

	}

}

pub fn exec<T: Send + Clone + 'static, F: FnOnce() -> T + Send + 'static>(f: F) -> Task<T> {

	let (tx, rx) = mpsc::channel();

	let t = thread::spawn(move || {
		tx.send(f());
	});

	return Task::from_rx(rx);

}

pub struct Task<T> {
	rx: Receiver<T>,
	data: Option<T>,
}

impl<T> Task<T> {

	fn from_rx(rx: Receiver<T>) -> Self {
		return Self {
			rx: rx,
			data: None,
		};
	}

	pub fn get(&mut self) -> &Option<T> {
		if let Ok(data) = self.rx.try_recv() {
			self.data = Some(data);
		}
		return &self.data;
	}

}

#[macro_export]
macro_rules! task {
	($val:expr) => {
		task::exec(|| {
			return $val;
		})
	}
}

