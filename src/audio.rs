// wengwengweng

//! Handles sounds

use rodio::Source;
use crate::*;

// context
ctx!(AUDIO: AudioCtx);

struct AudioCtx {
	device: rodio::Device,
}

pub fn init() {

	let device = rodio::default_output_device().unwrap();

	ctx_init(AudioCtx {
		device: device,
	});

}

pub fn enabled() -> bool {
	return ctx_is_ok();
}

pub fn play(track: &Track) {

	let audio = ctx_get();
	let sink = &track.sink;
	let data = track.cursor.clone();
	let src = rodio::Decoder::new(data).unwrap().convert_samples();

	rodio::play_raw(&audio.device, src);

}

pub fn pause(track: &Track) {
	track.sink.pause();
}

pub struct Track {

	sink: rodio::Sink,
	cursor: std::io::Cursor<&'static [u8]>

}

impl Track {

	pub fn from_bytes(data: &'static [u8]) -> Self {

		let audio = ctx_get();
		let sink = rodio::Sink::new(&audio.device);
		let cursor = std::io::Cursor::new(data);

		return Self {
			sink: sink,
			cursor: cursor,
		};

	}

}

