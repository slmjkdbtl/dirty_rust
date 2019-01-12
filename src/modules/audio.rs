// wengwengweng

//! Sound loading & playback

use std::io::Cursor;
use std::time::Duration;

use rodio::Source;
use rodio::Decoder;
use rodio::Sink;
use rodio::source::Buffered;

use crate::*;

const MAX_STATE_STACK: usize = 8;

// context
ctx!(AUDIO: AudioCtx);

struct AudioCtx {

	device: rodio::Device,
	state: State,
	state_stack: Vec<State>,

}

#[derive(Clone, Copy)]
struct State {
	speed: f32,
	amplify: f32,
}

impl Default for State {
	fn default() -> Self {
		return Self {
			speed: 1.0,
			amplify: 1.0,
		}
	}
}

/// initialize audio module
pub fn init() {

	if !app::enabled() {
		panic!("can't init audio without app");
	}

	ctx_init(AudioCtx {

		device: rodio::default_output_device().expect("failed to get audio device"),
		state: State::default(),
		state_stack: Vec::with_capacity(MAX_STATE_STACK),

	});

}

/// check if audio module is initialized
pub fn enabled() -> bool {
	return ctx_is_ok();
}

/// play a given sound once till end
pub fn play(sound: &Sound) {

	let ctx = ctx_get();

	rodio::play_raw(
		&ctx.device,
		sound.buffer
			.clone()
			.speed(ctx.state.speed)
			.amplify(ctx.state.amplify)
			.convert_samples()
	);

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

/// a track has more control
pub struct Track {
	sink: Sink,
}

/// play a sound and return a track
pub fn track(sound: &Sound) -> Track {

	let ctx = ctx_get();
	let sink = Sink::new(&ctx.device);

	sink.append(
		sound.buffer
			.clone()
			.speed(ctx.state.speed)
			.amplify(ctx.state.amplify)
	);

	return Track {
		sink: sink,
	}

}

/// set global speed
pub fn speed(s: f32) {
	assert!(s > 0.0 && s < 2.0, "invalid speed");
	ctx_get_mut().state.speed = s;
}

/// set global amplify
pub fn amplify(s: f32) {
	assert!(s >= 0.0 && s < 2.0, "invalid amplify");
	ctx_get_mut().state.amplify = s;
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

/// push state
pub fn push() {

	let audio = ctx_get_mut();
	let stack = &mut audio.state_stack;

	if (stack.len() < MAX_STATE_STACK) {
		stack.push(audio.state);
	} else {
		panic!("cannot push anymore");
	}

}

/// pop state
pub fn pop() {

	let mut audio = ctx_get_mut();
	let stack = &mut audio.state_stack;

	audio.state = stack.pop().expect("cannot pop anymore");

}

/// reset state
pub fn reset() {

	let gfx_mut = ctx_get_mut();

	gfx_mut.state_stack.clear();
	gfx_mut.state = State::default();

}

