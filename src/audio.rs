// wengwengweng

//! Audio Playback
//!
//! This module provides 2 types of high-level types:
//!  - [`Sound`](struct.Sound.html), buffered audio mainly for sound effects
//!  - [`Track`](struct.Track.html), streamed audio mainly for music

// TODO: loop
// TODO: sample rate conversion

use crate::Result;
use crate::math::*;

mod utils;

import!(mixer);
import!(vorbis);
import!(wav);
import!(mp3);
import!(decoder);
import!(buffer);
import!(converter);
export!(source);
export!(types);
export!(effect);

#[cfg(not(web))]
export!(track);
#[cfg(not(web))]
export!(sound);

#[cfg(not(web))]
export!(native);
#[cfg(web)]
export!(web);

pub mod music;
pub mod synth;

const SAMPLE_RATE: u32 = 44100;
const CHANNEL_COUNT: ChannelCount = ChannelCount::Two;

/// Represents A Frame in Audio
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
	left: f32,
	right: f32,
}

impl Frame {
	pub fn new(l: f32, r: f32) -> Self {
		return Self {
			left: l,
			right: r,
		};
	}
	pub fn from_i16(l: i16, r: i16) -> Self {
		return Self {
			left: l as f32 / i16::MAX as f32,
			right: r as f32 / i16::MAX as f32,
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

use std::ops::*;

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
	channel_count: u8,
	sample_rate: u32,
}

