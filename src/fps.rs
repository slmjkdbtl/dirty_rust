// wengwengweng

use std::time::Duration;

pub struct FPSCounter {
	frames: usize,
	timer: Duration,
	fps: u16,
}

impl FPSCounter {

	pub fn new() -> Self {
		return Self {
			frames: 0,
			timer: Duration::from_secs(0),
			fps: 0,
		}
	}

	pub fn tick(&mut self, dt: Duration) {

		self.frames += 1;
		self.timer += dt;

		if self.timer.as_secs_f32() >= 1.0 {
			self.fps = self.frames as u16;
			self.timer = Duration::from_secs(0);
			self.frames = 0;
		}

	}

	pub fn fps(&self) -> u16 {
		return self.fps;
	}

}

