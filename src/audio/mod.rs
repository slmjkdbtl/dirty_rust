// wengwengweng

//! Audio Playback
//!
//! This module provides 2 types of high-level types:
//!  - [`Sound`](struct.Sound.html), buffered audio mainly for sound effects
//!  - [`Track`](struct.Track.html), streamed audio mainly for music

// TODO: sample rate conversion

use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;

use math::*;

mod utils;

import!(mixer);
import!(vorbis);
import!(wav);
import!(mp3);
import!(decoder);
import!(buffer);
import!(resample);
export!(source);
export!(types);
export!(effect);
export!(spatial);
#[cfg(not(web))]
export!(track);
#[cfg(not(web))]
export!(sound);

#[cfg(not(web))]
export!(native);
#[cfg(web)]
export!(web);

use crate::*;

pub mod music;
pub mod synth;

pub const SPEC: Spec = Spec {
	sample_rate: 44100,
	channel_count: 2,
};

