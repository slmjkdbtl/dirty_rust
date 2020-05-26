// wengwengweng

use std::time::Duration;
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

		return Ok(Self {
			buffer,
			mixer: Arc::clone(ctx.mixer()),
		});

	}

	/// play sound
	pub fn play(&self) -> Result<()> {

		let mut mixer = self.mixer
			.lock()
			.map_err(|_| "failed to get mixer".to_string())?;

		mixer.add(Arc::new(Mutex::new(self.buffer.clone())))?;

		return Ok(());

	}

	/// returns a [`SoundBuilder`](SoundBuilder) that plays sound with config
	pub fn builder(&self) -> SoundBuilder {
		return SoundBuilder {
			buffer: Arc::new(Mutex::new(self.buffer.clone())),
			mixer: &self.mixer,
			effects: vec![],
		};
	}

	/// get duration
	pub fn duration(&self) -> Duration {
		return self.buffer.duration();
	}

}

/// A Builder for Playing [`Sound`](Sound) with Configs
pub struct SoundBuilder<'a> {
	buffer: Arc<Mutex<Buffered>>,
	effects: Vec<Arc<Mutex<dyn Effect + Send>>>,
	mixer: &'a Arc<Mutex<Mixer>>,
}

impl<'a> SoundBuilder<'a> {

	pub fn add(mut self, e: impl Effect + Send + 'static) -> Self {
		self.effects.push(Arc::new(Mutex::new(e)));
		return self;
	}

	pub fn pan(self, p: f32) -> Self {
		return self.add(Pan::new(p));
	}

	pub fn volume(self, v: f32) -> Self {
		return self.add(Volume::new(v));
	}

	pub fn delay(self, v: usize, d: f32) -> Self {
		return self.add(Delay::new(v, d));
	}

	pub fn play(self) -> Result<()> {

		let mut mixer = self.mixer
			.lock()
			.map_err(|_| "failed to get mixer".to_string())?;

		let id = mixer.add(self.buffer)?;

		for e in self.effects {
			mixer.add_effect(&id, e);
		}

		return Ok(());

	}

}

