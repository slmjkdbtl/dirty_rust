// wengwengweng

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Envelope {
	pub attack: f32,
	pub decay: f32,
	pub sustain: f32,
	pub release: f32,
}

impl Default for Envelope {
	fn default() -> Self {
		return Self {
			attack: 0.0,
			decay: 0.0,
			sustain: 1.0,
			release: 0.0,
		};
	}
}

