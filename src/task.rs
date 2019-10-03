// wengwengweng

//! Simple Threading Utilities

use std::sync::mpsc;
use std::thread;

pub fn exec<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> Task<T> {
	return Task::new(f);
}

pub struct Task<T: Send + 'static> {
	rx: mpsc::Receiver<T>,
	data: Option<T>,
	done: bool,
	thread: thread::JoinHandle<()>,
}

impl<T: Send + 'static> Task<T> {

	pub fn new(f: impl FnOnce() -> T + Send + 'static) -> Self {

		let (tx, rx) = mpsc::channel();

		// TODO: deal with error inside thread::spawn
		let t = thread::spawn(move || {
			tx.send(f()).expect("thread failure");
		});

		return Self {
			rx: rx,
			thread: t,
			data: None,
			done: false,
		};

	}

	pub fn done(&self) -> bool {
		return self.done;
	}

	pub fn block(&mut self) -> Option<T> {
		self.done = true;
		return self.rx.recv().ok();
	}

	pub fn poll(&mut self) -> Option<T> {

		if self.done {
			return None;
		}

		if let Ok(data) = self.rx.try_recv() {
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

#[macro_export]
macro_rules! task {
	($val:expr) => {
		task::exec(|| {
			return $val;
		})
	}
}

