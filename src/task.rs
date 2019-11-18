// wengwengweng

//! Simple Threading Utilities

use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

pub trait TaskItem = Send + 'static;
pub trait TaskAction<T: TaskItem> = FnOnce() -> T + Send + 'static;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TaskPhase {
	StandBy,
	Working,
	Done,
}

pub struct TaskPool<T: TaskItem> {
	queue: VecDeque<Task<T>>,
	active: Vec<Task<T>>,
	max: usize,
	completed: usize,
	total: usize,
}

impl<T: TaskItem> Default for TaskPool<T> {
	fn default() -> Self {
		return Self::new(num_cpus::get());
	}
}

impl<T: TaskItem> TaskPool<T> {

	pub fn new(max: usize) -> Self {
		return Self {
			queue: VecDeque::new(),
			active: vec![],
			max: max,
			completed: 0,
			total: 0,
		};
	}

	pub fn exec(&mut self, f: impl FnOnce() -> T + TaskItem) {

		self.queue.push_back(Task::new(f));
		self.adjust();
		self.total += 1;

	}

	fn adjust(&mut self) {

		self.active.retain(|t| t.phase() != TaskPhase::Done);

		for _ in 0..self.max as usize - self.active.len() {
			if let Some(mut task) = self.queue.pop_front() {
				task.start();
				self.active.push(task);
			}
		}

	}

	pub fn poll(&mut self) -> Vec<T> {

		let mut basket = vec![];

		for task in &mut self.active {
			if let Some(data) = task.poll() {
				self.completed += 1;
				basket.push(data);
			}
		}

		self.adjust();

		return basket;

	}

	pub fn clear_queue(&mut self) {
		self.queue.clear();
	}

	pub fn queue_count(&self) -> usize {
		return self.queue.len();
	}

	pub fn active_count(&self) -> usize {
		return self.active.len();
	}

	pub fn completed(&self) -> usize {
		return self.completed;
	}

	pub fn total(&self) -> usize {
		return self.total;
	}

}

pub struct Task<T: TaskItem> {
	rx: Option<mpsc::Receiver<T>>,
	action: Option<Box<dyn TaskAction<T>>>,
	phase: TaskPhase,
}

impl<T: TaskItem> Task<T> {

	pub fn new(f: impl FnOnce() -> T + TaskItem) -> Self {
		return Self {
			action: Some(box f),
			rx: None,
			phase: TaskPhase::StandBy,
		};
	}

	pub fn exec(f: impl FnOnce() -> T + TaskItem) -> Self {

		let mut task = Self::new(f);

		task.start();

		return task;

	}

	pub fn start(&mut self) {

		if let Some(action) = self.action.take() {

			let (tx, rx) = mpsc::channel();

			// TODO: deal with error inside thread::spawn
			thread::spawn(move || {
				tx.send(action()).expect("thread failure");
			});

			self.rx = Some(rx);
			self.phase = TaskPhase::Working;

		}

	}

	pub fn started(&self) -> bool {
		return self.rx.is_some();
	}

	pub fn phase(&self) -> TaskPhase {
		return self.phase;
	}

	pub fn poll_blocked(&mut self) -> Option<T> {

		let rx = self.rx.as_ref()?;
		let data = rx.recv().ok()?;

		self.phase = TaskPhase::Done;

		return Some(data);

	}

	pub fn poll(&mut self) -> Option<T> {

		let rx = self.rx.as_ref()?;

		if self.phase != TaskPhase::Working {
			return None;
		}

		let data = rx.try_recv().ok()?;

		self.phase = TaskPhase::Done;

		return Some(data);

	}

}

