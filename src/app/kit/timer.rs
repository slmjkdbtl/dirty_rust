// wengwengweng

#[derive(Clone, Copy, Debug)]
pub struct Timer {
	time: f32,
	limit: f32,
	done: bool,
}

impl Timer {

	pub fn new(time: f32,) -> Self {
		return Self {
			time: 0.0,
			limit: time,
			done: false,
		}
	}

	pub fn reset(&mut self) {
		self.reset_to(self.limit);
	}

	pub fn reset_to(&mut self, time: f32,) {

		self.time = 0.0;
		self.limit = time;
		self.done = false;

	}

	pub fn tick(&mut self, dt: f32) -> bool {

		self.time += dt;

		if self.time >= self.limit {
			self.done = true;
			return true;
		}

		return false;

	}
}

