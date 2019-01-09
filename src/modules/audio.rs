// wengwengweng

//! Sound loading & playback

use std::io::Cursor;

use rodio::Source;
use rodio::Decoder;
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
	rodio::play_raw(&ctx_get().device, sound.buffer.clone().convert_samples());
}

/// a sound is meant to play just once
pub struct Sound {
	/// buffer
	buffer: Buffered<Decoder<Cursor<Vec<u8>>>>,
}

impl Sound {

	/// create a sound from bytes
	pub fn from_bytes(data: &[u8]) -> Self {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor).expect("failed to decode sound");

		return Self {
			buffer: source.buffered(),
		};

	}

	/// create a sound from file
	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

}

