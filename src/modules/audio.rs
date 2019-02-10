// wengwengweng

//! Sound Loading & playback

use std::io::Cursor;
use std::time::Duration;

use rodio::Source;
use rodio::Decoder;
use rodio::Sink;
use rodio::source::Buffered;

use crate::*;

// context
ctx!(AUDIO: AudioCtx);

struct AudioCtx {
	device: rodio::Device,
}

/// initialize audio module
pub fn init() {

	if !app::enabled() {
		panic!("can't init audio without app");
	}

	ctx_init(AudioCtx {
		device: rodio::default_output_device().expect("failed to get audio device"),
	});

}

/// check if audio module is initialized
pub fn enabled() -> bool {
	return ctx_ok();
}

/// play a given sound once till end
pub fn play(sound: &Sound) {
	rodio::play_raw(&ctx_get().device, sound.apply().convert_samples());
}

/// base struct containing sound data and effects data
#[derive(Clone)]
pub struct Sound {
	buffer: Buffered<Decoder<Cursor<Vec<u8>>>>,
	effect: Effect,
}

#[derive(Clone, Copy)]
struct Effect {
	speed: f32,
	volume: f32,
	repeat: bool,
	fadein: u64,
}

impl Default for Effect {
	fn default() -> Self {
		return Self {
			speed: 1.0,
			volume: 1.0,
			repeat: false,
			fadein: 0,
		};
	}
}

impl Sound {

	/// create a sound from bytes
	pub fn from_bytes(data: &[u8]) -> Self {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor).expect("failed to decode sound");

		return Self {
			buffer: source.buffered(),
			effect: Effect::default(),
		};

	}

	/// create a sound from file
	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

	/// return a new sound with given speed
	pub fn speed(&self, s: f32) -> Self {
		assert!(s > 0.0 && s <= 2.0, "invalid speed");
		return Self {
			effect: Effect {
				speed: s,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	/// return a new sound with given volume
	pub fn volume(&self, v: f32) -> Self {
		assert!(v >= 0.0 && v <= 2.0, "invalid volume");
		return Self {
			effect: Effect {
				volume: v,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	/// return a new sound that would repeat infinitely
	pub fn repeat(&self) -> Self {
		return Self {
			effect: Effect {
				repeat: true,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	/// return a new sound with given fadein time
	pub fn fadein(&self, time: u64) -> Self {
		return Self {
			effect: Effect {
				fadein: time,
				.. self.effect
			},
			buffer: self.buffer.clone(),
		}
	}

	fn apply(&self) -> Box<dyn Source<Item = i16> + Send> {

		type S = dyn Source<Item = i16> + Send;
		let s = Box::new(self.buffer.clone());
		let effect = self.effect;

		let s: Box<S> = if effect.speed != 0.0 {
			Box::new(s.speed(effect.speed))
		} else {
			s
		};

		let s: Box<S> = if effect.volume != 0.0 {
			Box::new(s.amplify(effect.volume))
		} else {
			s
		};

		let s: Box<S> = if effect.fadein != 0 {
			Box::new(s.fade_in(Duration::from_millis(effect.fadein)))
		} else {
			s
		};

		let s: Box<S> = if effect.repeat {
			Box::new(s.repeat_infinite())
		} else {
			s
		};

		return s;

	}

}

/// a track has more control
pub struct Track {
	sink: Sink,
}

/// play a sound and return a track
pub fn track(sound: &Sound) -> Track {

	let ctx = ctx_get();
	let sink = Sink::new(&ctx.device);

	sink.append(sound.apply());

	return Track {
		sink: sink,
	}

}

/// pause a track
pub fn pause(track: &Track) {
	track.sink.pause();
}

/// resume a track
pub fn resume(track: &Track) {
	track.sink.play();
}

/// drop a track
pub fn drop(track: Track) {
	track.sink.detach();
}

