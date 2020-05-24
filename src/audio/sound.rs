// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::io::Cursor;

use super::*;

/// Buffered Sound (mainly for short sound effects)
#[derive(Clone)]
pub struct Sound {
	buffer: Buffered,
	mixer: Arc<Mutex<Mixer>>,
}

impl Sound {

	/// create sound from bytes of an audio file
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let buffer = Buffered::new(Decoder::new(Cursor::new(data.to_owned()))?);

		let t = Self {
			buffer: buffer,
			mixer: Arc::clone(ctx.mixer()),
		};

		return Ok(t);

	}

	/// play sound
	pub fn play(&self) {
		if let Ok(mut mixer) = self.mixer.lock() {
			mixer.add(Arc::new(Mutex::new(self.buffer.clone())));
		}
	}

	/// returns a [`SoundBuilder`](SoundBuilder) that plays sound with config
	pub fn builder(&self) -> SoundBuilder {
		return SoundBuilder {
			ctrl: Control::default(),
			buffer: Arc::new(Mutex::new(self.buffer.clone())),
			mixer: &self.mixer,
		};
	}

}

/// A Builder for Playing [`Sound`](Sound) with Configs
pub struct SoundBuilder<'a> {
	ctrl: Control,
	buffer: Arc<Mutex<Buffered>>,
	mixer: &'a Arc<Mutex<Mixer>>,
}

impl<'a> SoundBuilder<'a> {
	pub fn pan(mut self, p: f32) -> Self {
		self.ctrl.pan = p;
		return self;
	}
	pub fn volume(mut self, v: f32) -> Self {
		self.ctrl.volume = v;
		return self;
	}
	pub fn play(self) {
		if let Ok(mut mixer) = self.mixer.lock() {
			mixer.add(self.buffer);
		}
	}
}

