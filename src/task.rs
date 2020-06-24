// wengwengweng

//! Simple Threading / Resource Loading Abstraction

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use once_cell::sync::Lazy;

use crate::Result;

type Job = Box<dyn FnOnce() + Send + 'static>;

static DEFAULT_POOL: Lazy<ThreadPool> = Lazy::new(|| {
	return ThreadPool::new(1)
		.expect("failed to init loader thread pool");
});

/// Simple ThreadPool
pub struct ThreadPool {
	job_rx: Arc<Mutex<mpsc::Receiver<Job>>>,
	job_tx: Mutex<mpsc::Sender<Job>>,
}

impl ThreadPool {

	/// create a new thread pool, specifying the number of worker threads
	pub fn new(count: usize) -> Result<Self> {

		let (job_tx, job_rx) = mpsc::channel::<Job>();
		let job_rx = Arc::new(Mutex::new(job_rx));

		for _ in 0..count {
			let job_rx_t = job_rx.clone();
			thread::Builder::new()
				.name(format!("dirty_threadpool"))
				.spawn(move || {
					loop {
						let res = || -> Result<()> {
							let job = job_rx_t
								.lock()
								.map_err(|_| format!("failed to lock job receiver"))?
								.recv()
								.map_err(|_| format!("failed to lock job receiver"))?;
							job();
							return Ok(());
						}();
						if let Err(e) = res {
							elog!("{}", e);
						}
					}
				})
				.map_err(|_| format!("failed to spawn thread"))?;
		}

		return Ok(Self {
			job_rx: job_rx,
			job_tx: Mutex::new(job_tx),
		});

	}

	/// execute a task
	pub fn exec(&self, job: impl FnOnce() + Send + 'static) -> Result<()> {

		return self.job_tx
			.lock()
			.map_err(|_| format!("failed to lock job sender"))?
			.send(Box::new(job))
			.map_err(|_| format!("failed to send job"));

	}

}

/// Execute a Task Using the Global Thread Pool
pub struct Task<T: Send + 'static> {
	rx: mpsc::Receiver<T>,
	done: bool,
}

impl<T: Send + 'static> Task<T> {

	/// start a new task with custom thread pool
	pub fn with(pool: &ThreadPool, f: impl FnOnce() -> T + Send + 'static) -> Result<Self> {

		let (tx, rx) = mpsc::channel();

		pool.exec(move || {
			if let Err(e) = tx.send(f()) {
				elog!("failed to execute task");
			};
		})?;

		return Ok(Self {
			rx: rx,
			done: false,
		});

	}

	/// start a new task with default thread pool
	pub fn new(f: impl FnOnce() -> T + Send + 'static) -> Result<Self> {
		return Self::with(&DEFAULT_POOL, f);
	}

	/// if it's done loading
	pub fn done(&self) -> bool {
		return self.done;
	}

	/// block until the resource is ready
	pub fn poll_blocked(&mut self) -> Option<T> {
		let data = self.rx.recv().ok()?;
		self.done = true;
		return Some(data);
	}

	/// check if the resource is ready
	pub fn poll(&mut self) -> Option<T> {

		let data = self.rx
			.try_recv()
			.ok()?;

		self.done = true;

		return Some(data);

	}

}

