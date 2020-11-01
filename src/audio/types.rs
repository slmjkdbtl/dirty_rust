// wengwengweng

use std::ops::*;

/// Represents A Frame in Audio
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
	pub left: f32,
	pub right: f32,
}

impl Frame {
	pub fn new(l: f32, r: f32) -> Self {
		return Self {
			left: l,
			right: r,
		};
	}
	pub fn mono(v: f32) -> Self {
		return Self {
			left: v,
			right: v,
		};
	}
	pub fn zero() -> Self {
		return Self {
			left: 0.0,
			right: 0.0,
		};
	}
	pub fn clamp(self) -> Self {
		return Self {
			left: self.left.max(-1.0).min(1.0),
			right: self.right.max(-1.0).min(1.0),
		};
	}
}

impl Default for Frame {
	fn default() -> Self {
		return Self {
			left: 0.0,
			right: 0.0,
		};
	}
}

impl Add for Frame {

	type Output = Self;

	fn add(self, other: Self) -> Self {
		return Self {
			left: self.left + other.left,
			right: self.right + other.right,
		};
	}

}

impl Sub for Frame {

	type Output = Self;

	fn sub(self, other: Self) -> Self {
		return Self {
			left: self.left - other.left,
			right: self.right - other.right,
		};
	}

}

impl Mul<f32> for Frame {

	type Output = Self;

	fn mul(self, f: f32) -> Self {
		return Self {
			left: self.left * f,
			right: self.right * f,
		};
	}

}

impl Mul<Pan> for Frame {

	type Output = Self;

	fn mul(self, p: Pan) -> Self {
		return Self {
			left: self.left * p.left,
			right: self.right * p.right,
		};
	}

}

impl Div<f32> for Frame {

	type Output = Self;

	fn div(self, f: f32) -> Self {
		return Self {
			left: self.left / f,
			right: self.right / f,
		};
	}

}

impl AddAssign for Frame {
	fn add_assign(&mut self, other: Frame) {
		*self = *self + other;
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spec {
	pub channel_count: u16,
	pub sample_rate: u32,
}

/// Influence Left & Right Channel Volume
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pan {
	pub left: f32,
	pub right: f32,
}

impl Pan {
	pub fn new(l: f32, r: f32) -> Self {
		return Self {
			left: l,
			right: r,
		};
	}
}

impl Default for Pan {
	fn default() -> Self {
		return Self {
			left: 1.0,
			right: 1.0,
		};
	}
}

impl Mul<f32> for Pan {

	type Output = Self;

	fn mul(self, f: f32) -> Self {
		return Self {
			left: self.left * f,
			right: self.right * f,
		};
	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Channel {
	Left,
	Right,
}

impl Channel {
	fn as_usize(&self) -> usize {
		return match self {
			Channel::Left => 0,
			Channel::Right => 1,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChannelCount {
	One,
	Two,
}

