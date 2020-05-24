// wengwengweng

use super::*;

pub trait Effect {
	fn frame(&mut self, _: Frame) -> Frame;
}

#[derive(Clone)]
pub struct Volume(pub f32);

impl Effect for Volume {
	fn frame(&mut self, (left, right): Frame) -> Frame {
		return (left * self.0, right * self.0);
	}
}

#[derive(Clone)]
pub struct Pan(pub f32);

impl Effect for Pan {
	fn frame(&mut self, (left, right): Frame) -> Frame {
		return (
			left * self.0.map(1.0, -1.0, 0.0, 2.0),
			right * self.0.map(-1.0, 1.0, 0.0, 2.0),
		);
	}
}

