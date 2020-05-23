// wengwengweng

//! Audio Playback

use crate::Result;

import!(mixer);
import!(utils);
import!(wav);
import!(vorbis);
import!(mp3);
import!(decoder);
import!(buffer);
export!(source);

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

