// wengwengweng

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Life {
	life: f32,
	afterlife: f32,
	released: bool,
	dead: bool,
	volume: f32,
	envelope: Envelope,
}

impl Life {

	pub fn new(e: Envelope) -> Self {
		return Self {
			life: 0.0,
			afterlife: 0.0,
			released: false,
			dead: false,
			volume: 0.0,
			envelope: e,
		};
	}

	pub fn update(&mut self, dt: f32) {

		if !self.released {
			self.life += dt;
		} else {
			self.afterlife += dt;
		}

		let e = self.envelope;

		if !self.released {
			if self.life < e.attack {
				self.volume = self.life / e.attack;
			} else if self.life >= e.attack && self.life < e.attack + e.decay {
				self.volume = 1.0 - (self.life - e.attack) / e.decay * (1.0 - e.sustain);
			} else {
				self.volume = e.sustain;
			}
		} else {
			if e.release == 0.0 {
				self.volume = 0.0;
			} else {
				self.volume = e.sustain - (self.afterlife / e.release) * e.sustain;
			}
		}

		if self.released {
			if self.afterlife > e.release {
				self.volume = 0.0;
				self.dead = true;
			}
		}

	}

	pub fn release(&mut self) {
		self.released = true;
	}

	pub fn dead(&self) -> bool {
		return self.dead;
	}

	pub fn volume(&self) -> f32 {
		return self.volume;
	}

}

