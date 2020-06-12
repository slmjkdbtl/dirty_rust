// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;

use super::*;

/// Buffered Sound (mainly for short sound effects)
#[derive(Clone)]
pub struct Sound {
	playback: AudioBufferPlayback,
	mixer: Arc<Mutex<Mixer>>,
}

impl Sound {

	/// create sound from bytes of an audio file
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let buffer = AudioBuffer::from_bytes(data)?;
		let playback = AudioBufferPlayback::new(buffer);

		return Ok(Self {
			playback: playback,
			mixer: Arc::clone(ctx.mixer()),
		});

	}

	/// play sound
	pub fn play(&self) -> Result<()> {

		let mut mixer = self.mixer
			.lock()
			.map_err(|_| format!("failed to get mixer"))?;

		mixer.add(Arc::new(Mutex::new(self.playback.clone())))?;

		return Ok(());

	}

	/// returns a [`SoundBuilder`](SoundBuilder) that plays sound with config
	pub fn builder(&self) -> SoundBuilder {
		return SoundBuilder {
			playback: self.playback.clone(),
			mixer: &self.mixer,
			effects: vec![],
			volume: 1.0,
			pan: Pan::new(1.0, 1.0),
		};
	}

}

/// A Builder for Playing [`Sound`](Sound) with Configs
pub struct SoundBuilder<'a> {
	playback: AudioBufferPlayback,
	effects: Vec<Arc<Mutex<dyn Effect + Send>>>,
	mixer: &'a Arc<Mutex<Mixer>>,
	volume: f32,
	pan: Pan,
}

impl<'a> SoundBuilder<'a> {

	pub fn chain(mut self, e: impl Effect + Send + 'static) -> Self {
		self.effects.push(Arc::new(Mutex::new(e)));
		return self;
	}

	pub fn pan(mut self, l: f32, r: f32) -> Self {
		self.pan = Pan::new(l, r);
		return self;
	}

	pub fn volume(mut self, v: f32) -> Self {
		self.volume = v;
		return self;
	}

	pub fn play(self) -> Result<()> {

		let mut mixer = self.mixer
			.lock()
			.map_err(|_| format!("failed to get mixer"))?;

		let id = mixer.add(Arc::new(Mutex::new(self.playback)))?;

		for e in self.effects {
			mixer.add_effect(&id, e);
		}

		let control = mixer
			.get_control(&id)
			.ok_or(format!("failed to get sound control"))?;

		if let Ok(mut ctrl) = control.lock() {
			ctrl.pan = self.pan;
			ctrl.volume = self.volume;
		}

		return Ok(());

	}

}

#[derive(Clone)]
struct AudioBufferPlayback {
	buffer: Arc<AudioBuffer>,
	cur_pos: usize,
}

impl AudioBufferPlayback {
	pub fn new(buffer: AudioBuffer) -> Self {
		return AudioBufferPlayback {
			buffer: Arc::new(buffer),
			cur_pos: 0,
		};
	}
}

impl Iterator for AudioBufferPlayback {

	type Item = Frame;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(frame) = self.buffer.frames().get(self.cur_pos) {
			self.cur_pos += 1;
			return Some(*frame);
		}
		return None;
	}

}

impl Source for AudioBufferPlayback {

	fn sample_rate(&self) -> u32 {
		return self.buffer.sample_rate();
	}

	fn seek_start(&mut self) -> Result<()> {
		self.cur_pos = 0;
		return Ok(());
	}

}

