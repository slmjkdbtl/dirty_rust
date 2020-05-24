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

		let mut mixer = ctx.mixer()
			.lock()
			.map_err(|_| format!("failed to get mixer"))?;

		let id = mixer.add(src.clone());

		let control = mixer
			.get_control(&id)
			.ok_or(format!("failed to get mixer"))?;

		mixer.add_effect(&id, volume.clone());
		mixer.add_effect(&id, pan.clone());
		control.set_paused(true);

		return Ok(Self {
			src: src,
			id: id,
			control: control,
			volume: volume,
			pan: pan,
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

	pub fn reset(&self) -> Result<()> {
		if let Ok(mut src) = self.src.lock() {
			src.reset()?;
		}
		return Ok(());
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
		return self.control.paused();
	}

}

