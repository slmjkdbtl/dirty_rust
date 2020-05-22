// wengwengweng

use std::rc::Rc;
use std::io::Cursor;

use rodio::Source;
use rodio::Decoder;
use rodio::Sink;
use rodio::source::Buffered;

use crate::*;

/// The Audio Context. See [mod-level doc](audio) for usage.
pub struct Audio {
	device: Rc<rodio::Device>,
}

impl Audio {
	pub(crate) fn new() -> Result<Self> {
		let device = rodio::default_output_device().ok_or(format!("failed to get audio device"))?;
		return Ok(Self {
			device: Rc::new(device),
		});
	}
}

/// One-shot Sound
#[derive(Clone)]
pub struct Sound {
	buffer: Buffered<Decoder<Cursor<Vec<u8>>>>,
	device: Rc<rodio::Device>,
}

impl Sound {

	/// create sound from bytes of an audio file
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let cursor = Cursor::new(data.to_owned());
		let source = Decoder::new(cursor)
			.map_err(|_| format!("failed to parse sound from file"))?;

		return Ok(Self {
			buffer: source.buffered(),
			device: ctx.device.clone(),
		});

	}

	/// play sound
	pub fn play(&self) -> Result<()> {
		rodio::play_raw(&self.device, self.buffer.clone().convert_samples());
		return Ok(());
	}

}

/// Streamed Audio That Can Pause / Seek
pub struct Track {
	sink: Sink,
}

impl Track {

	/// create sound from bytes of an audio file
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {
		return Self::from_sound(Sound::from_bytes(ctx, data)?);
	}

	pub fn from_sound(sound: Sound) -> Result<Self> {

		let device = rodio::default_output_device().ok_or(format!("failed to get audio device"))?;
		let sink = Sink::new(&device);

		sink.append(sound.buffer);
		sink.pause();

		return Ok(Self {
			sink,
		});

	}

	/// play / resume track
	pub fn play(&self) {
		self.sink.play();
	}

	/// pause track
	pub fn pause(&self) {
		self.sink.pause();
	}

	/// check if is playing
	pub fn is_playing(&self) -> bool {
		return !self.sink.is_paused();
	}

	/// free track
	pub fn free(self) {
		self.sink.stop();
		self.sink.detach();
	}

}

