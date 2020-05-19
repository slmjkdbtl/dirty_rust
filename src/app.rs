// wengwengweng

use std::time::Duration;

use instant::Instant;

use crate::*;
use fps::*;

pub struct App {
	last_frame_time: Instant,
	fps_counter: FPSCounter,
	time: Duration,
	dt: Duration,
}

impl App {
	pub(crate) fn new() -> Self {
		return Self {
			time: Duration::from_secs_f32(0.0),
			dt: Duration::from_secs_f32(0.0),
			fps_counter: FPSCounter::new(),
			last_frame_time: Instant::now(),
		};
	}
}

impl App {

	pub(crate) fn tick(&mut self) {
		self.dt = self.last_frame_time.elapsed();
		self.fps_counter.tick(self.dt);
		self.last_frame_time = Instant::now();
	}

	pub fn time(&self) -> Duration {
		return self.time;
	}

	pub fn dt(&self) -> Duration {
		return self.dt;
	}

	pub fn fps(&self) -> u16 {
		return self.fps_counter.fps();
	}

}

