// wengwengweng

//! Sound loading & playback

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
	return ctx_is_ok();
}

/// play a given sound once till end
pub fn play(sound: &Sound) {
	rodio::play_raw(&ctx_get().device, sound.apply().convert_samples());
}

/// base struct containing sound data and effects data
#[derive(Clone)]
pub struct Sound {
	buffer: Buffered<Decoder<Cursor<Vec<u8>>>>,
	speed: f32,
	amplify: f32,
	repeat: bool,
	fadein: u64,
}

impl Sound {

	/// create a sound from bytes
	pub fn from_bytes(data: &[u8]) -> Self {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor).expect("failed to decode sound");

		return Self {
			buffer: source.buffered(),
			speed: 1.0,
			amplify: 1.0,
			repeat: false,
			fadein: 0,
		};

	}

	/// create a sound from file
	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

	/// return a new sound with speed effect
	pub fn speed(&self, s: f32) -> Self {
		assert!(s > 0.0 && s <= 2.0, "invalid speed");
		let mut sound = self.clone();
		sound.speed = s;
		return sound;
	}

	/// return a new sound with speed effect
	pub fn amplify(&self, a: f32) -> Self {
		assert!(a >= 0.0 && a <= 2.0, "invalid amplify");
		let mut sound = self.clone();
		sound.amplify = a;
		return sound;
	}

	/// return a new sound with reverb effect
	pub fn repeat(&self) -> Self {
		let mut sound = self.clone();
		sound.repeat = true;
		return sound;
	}

	/// return a new sound with reverb effect
	pub fn fadein(&self, time: u64) -> Self {
		let mut sound = self.clone();
		sound.fadein = time;
		return sound;
	}

	fn apply(&self) -> Box<dyn Source<Item = i16> + Send> {

		type S = dyn Source<Item = i16> + Send;
		let s = Box::new(self.buffer.clone());

		let s: Box<S> = if self.speed != 0.0 {
			Box::new(s.speed(self.speed))
		} else {
			s
		};

		let s: Box<S> = if self.amplify != 0.0 {
			Box::new(s.amplify(self.amplify))
		} else {
			s
		};

		let s: Box<S> = if self.fadein != 0 {
			Box::new(s.fade_in(Duration::from_millis(self.fadein)))
		} else {
			s
		};

		let s: Box<S> = if self.repeat {
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

