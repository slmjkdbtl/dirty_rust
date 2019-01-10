// wengwengweng

//! Sound loading & playback

use std::io::Cursor;
use std::time::Duration;

use rodio::Source;
use rodio::Decoder;
use rodio::Sink;
use rodio::Sample;
use rodio::source::Buffered;
use rodio::source::Speed;
use rodio::source::Delay;

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
pub fn effect<S>(sound: &Sound<S>)
where
	S: Source<Item = i16> + Send + 'static, {
	rodio::play_raw(&ctx_get().device, sound.buffer.clone().convert_samples());
}

/// a sound is meant to play just once
pub struct Sound<I>
where
	I: Source,
	I::Item: Sample, {

	buffer: Buffered<I>,

}

pub(crate) type SoundData = Sound<Decoder<Cursor<Vec<u8>>>>;

impl SoundData {

	/// create a sound from bytes
	pub fn from_bytes(data: &[u8]) -> Self {

		let cursor = Cursor::new(data.to_owned());
		let decoder = Decoder::new(cursor).expect("failed to decode sound");

		return Self {
			buffer: decoder.buffered(),
		};

	}

	/// create a sound from file
	pub fn from_file(fname: &str) -> Self {
		return Self::from_bytes(&fs::read_bytes(fname));
	}

}

impl<I> Sound<I>
where
	I: Source,
	I::Item: Sample, {

	/// get a sound with changed speed
	pub fn speed(&self, f: f32) -> Sound<Speed<Buffered<I>>> {
		return Sound {
			buffer: self.buffer.clone().speed(f).buffered(),
		}
	}

	/// get a sound with delay effect
	pub fn delay(&self, f: u64) -> Sound<Delay<Buffered<I>>> {
		return Sound {
			buffer: self.buffer.clone().delay(Duration::from_millis(f)).buffered(),
		}
	}

}

/// a track has more control
pub struct Track {
	sink: Sink,
}

/// play a sound and return a track
pub fn play<S>(sound: &Sound<S>) -> Track
where
	S: Source<Item = i16> + Send + 'static, {

	let sink = Sink::new(&ctx_get().device);

	sink.append(sound.buffer.clone());

	return Track {
		sink: sink,
	}

}

