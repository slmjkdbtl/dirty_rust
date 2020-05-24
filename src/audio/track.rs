// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::io::Cursor;

use super::*;

/// Streamed Sound (mainly for music)
#[derive(Clone)]
pub struct Track {
	src: Arc<Mutex<dyn Source + Send>>,
	paused: Arc<Mutex<bool>>,
	pan: Arc<Mutex<Pan>>,
	volume: Arc<Mutex<Volume>>,
}

impl Track {

	/// create track from bytes of an audio file
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let src = Decoder::new(Cursor::new(data.to_owned()))?;
		let src = Arc::new(Mutex::new(src));

		let volume = Arc::new(Mutex::new(Volume(1.0)));
		let pan = Arc::new(Mutex::new(Pan(0.0)));

		let paused = ctx
			.mixer()
			.lock()
			.map_err(|_| format!("failed to get mixer"))?
			.add_ex_paused(src.clone(), vec![
				volume.clone(),
				pan.clone(),
			]);

		return Ok(Self {
			src: src,
			paused: paused,
			volume: volume,
			pan: pan,
		});

	}

	/// play / resume track
	pub fn play(&self) {
		if let Ok(mut paused) = self.paused.lock() {
			*paused = false;
		}
	}

	/// pause track
	pub fn pause(&self) {
		if let Ok(mut paused) = self.paused.lock() {
			*paused = true;
		}
	}

	/// set pan
	pub fn set_pan(&self, p: f32) {
		if let Ok(mut pan) = self.pan.lock() {
			*pan = Pan(p);
		}
	}

	/// set volume
	pub fn set_volume(&self, v: f32) {
		if let Ok(mut volume) = self.volume.lock() {
			*volume = Volume(v);
		}
	}

	/// check if is paused
	pub fn paused(&self) -> bool {
		return self.paused
			.lock()
			.map(|b| *b)
			.unwrap_or(true)
			;
	}

}

