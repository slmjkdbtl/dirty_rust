// wengwengweng

//! Audio Playback
//!
//! This module provides 2 types of high-level types:
//!  - [`Sound`](Sound), buffered audio mainly for sound effects
//!  - [`Track`](Track), streamed audio mainly for music

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
#[cfg(feature = "synth")]
pub mod synth;

const SAMPLE_RATE: u32 = 44100;
const CHANNEL_COUNT: ChannelCount = ChannelCount::Two;

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

