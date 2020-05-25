// wengwengweng

use instant::Instant;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Timer {
	elapsed: Duration,
	time: Duration,
	done: bool,
}

impl Timer {

	pub fn new(time: Duration) -> Self {
		return Self {
			elapsed: Duration::from_secs_f32(0.0),
			time,
			done: false,
		}
	}

	pub fn from_secs(time: f32) -> Self {
		return Self::new(Duration::from_secs_f32(time));
	}

	pub fn reset(&mut self) {
		self.reset_to(self.time);
	}

	pub fn reset_to(&mut self, time: Duration) {
		self.elapsed = Duration::from_secs_f32(0.0);
		self.time = time;
		self.done = false;
	}

	pub fn reset_to_secs(&mut self, time: f32) {
		self.reset_to(Duration::from_secs_f32(time));
	}

	pub fn progress(&self) -> f32 {
		return self.elapsed.as_secs_f32() / self.time.as_secs_f32();
	}

	pub fn done(&self) -> bool {
		return self.done;
	}

	pub fn tick(&mut self, dt: Duration) -> bool {

		self.elapsed += dt;

		if self.elapsed >= self.time {
			self.done = true;
			return true;
		}

		return false;

	}

}

#[derive(Clone, Copy)]
pub struct PTimer {
	start_time: Instant,
	pause_time: Option<Instant>,
	pause_duration: Duration,
}

impl PTimer {

	pub fn new() -> Self {
		return Self {
			start_time: Instant::now(),
			pause_time: None,
			pause_duration: Duration::from_millis(0),
		};
	}

	pub fn pause(&mut self) {
		self.pause_time = Some(Instant::now());
	}

	pub fn start(&mut self) {
		if let Some(t) = self.pause_time.take() {
			self.pause_duration += t.elapsed();
		}
	}

	pub fn time(&self) -> Duration {
		return match self.pause_time {
			Some(t) => self.start_time.elapsed() - t.elapsed() - self.pause_duration,
			None => self.start_time.elapsed() - self.pause_duration,
		};
	}

}

