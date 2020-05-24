// wengwengweng

//! Audio Playback

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

