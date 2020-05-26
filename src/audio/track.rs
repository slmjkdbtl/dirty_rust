// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::io::Cursor;

use super::*;

/// Streamed Sound (mainly for music)
#[derive(Clone)]
pub struct Track {
	id: SourceID,
	src: Arc<Mutex<Decoder<Cursor<Vec<u8>>>>>,
	control: Arc<Control>,
	effects: BasicEffectChain,
}

impl Track {

	/// create track from bytes of an audio file
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let src = Decoder::new(Cursor::new(data.to_owned()))?;
		let src = Arc::new(Mutex::new(src));

		let mut mixer = ctx.mixer()
			.lock()
			.map_err(|_| "failed to get mixer".to_string())?;

		let id = mixer.add(src.clone())?;

		let control = mixer
			.get_control(&id)
			.ok_or("failed to get mixer".to_string())?;

		control.set_paused(true);

		let effects = BasicEffectChain::new();

		for e in effects.chain() {
			mixer.add_effect(&id, e);
		}

		return Ok(Self {
			src,
			id,
			control,
			effects,
		});

	}

	/// play / resume track
	pub fn play(&mut self) {
		self.control.set_paused(false);
	}

	/// pause track
	pub fn pause(&mut self) {
		self.control.set_paused(true);
	}

	/// set pan
	pub fn set_pan(&self, p: f32) {
		self.effects.set_pan(p);
	}

	/// set volume
	pub fn set_volume(&self, v: f32) {
		self.effects.set_volume(v);
	}

	/// set delay
	pub fn set_delay(&self, len: usize, cycles: usize, d: f32) {
		self.effects.set_delay(len, cycles, d);
	}

	/// set looping
	pub fn set_looping(&self, l: bool) {
		self.control.set_looping(l);
	}

	/// check if is paused
	pub fn paused(&self) -> bool {
		return self.control.paused();
	}

	/// remove audio from mixer
	pub fn detach(&self) {
		return self.control.detach();
	}

}

