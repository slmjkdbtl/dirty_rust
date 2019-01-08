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

pub fn init() {

	if !app::enabled() {
		app::error("can't init audio without app");
	}

	if let Some(device) = rodio::default_output_device() {
		ctx_init(AudioCtx {
			device: device,
		});
	} else {
		app::error("cannot find audio device")
	}

}

pub fn enabled() -> bool {
	return ctx_is_ok();
}

pub fn play(sound: &Sound) {
	rodio::play_raw(&ctx_get().device, sound.buffer.clone().convert_samples());
}

pub struct Sound {
	buffer: Buffered<Decoder<Cursor<Vec<u8>>>>,
}

impl Sound {

	pub fn from_bytes(data: &[u8]) -> Self {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor).unwrap();

		return Self {
			buffer: source.buffered(),
		};

	}

	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

}

