// wengwengweng

//! Audio Playback
//!
//! This module provides 2 types of high-level types:
//!  - [`Sound`](Sound), buffered audio mainly for sound effects
//!  - [`Track`](Track), streamed audio mainly for music

use crate::Result;
use crate::math::*;

import!(mixer);
import!(utils);
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

#[cfg(feature = "synth")]
pub mod synth;

pub type Frame = (f32, f32);
const SAMPLE_RATE: SampleRate = SampleRate::Hz44100;
const CHANNEL_COUNT: ChannelCount = ChannelCount::Two;

