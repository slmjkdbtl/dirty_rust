// wengwengweng

use std::sync::Arc;
use std::sync::Mutex;
use std::io::Cursor;

use super::*;

/// Streamed Sound (mainly for music)
#[derive(Clone)]
pub struct Track {
	id: SourceID,
	src: Arc<Mutex<Decoder<Cursor<Vec<u8>>>>>,
	control: Arc<Mutex<Control>>,
	mixer: Arc<Mutex<Mixer>>,
}

impl Track {

	/// create track from bytes of an audio file
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let src = Decoder::new(Cursor::new(data.to_owned()))?;
		let src = Arc::new(Mutex::new(src));

		let mut mixer = ctx.mixer()
			.lock()
			.map_err(|_| format!("failed to get mixer"))?;

		let id = mixer.add(src.clone())?;

		let control = mixer
			.get_control(&id)
			.ok_or(format!("failed to get mixer"))?;

		control.lock().unwrap().paused = true;

		return Ok(Self {
			src: src,
			id: id,
			control: control,
			mixer: ctx.mixer().clone(),
		});

	}

	/// play / resume track
	pub fn play(&self) {
		self.control.lock().unwrap().paused = false;
	}

	/// pause track
	pub fn pause(&self) {
		self.control.lock().unwrap().paused = true;
	}

	/// set volume
	pub fn set_volume(&self, v: f32) {
		self.control.lock().unwrap().volume = v;
	}

	/// get volume
	pub fn volume(&self) -> f32 {
		return self.control.lock().unwrap().volume;
	}

	/// set pan
	pub fn set_pan(&self, l: f32, r: f32) {
		self.control.lock().unwrap().pan = Pan::new(l, r);
	}

	/// get pan
	pub fn pan(&self) -> Pan {
		return self.control.lock().unwrap().pan;
	}

	/// set looping
	pub fn set_looping(&self, l: bool) {
		self.control.lock().unwrap().looping = l;
	}

	/// check if is paused
	pub fn paused(&self) -> bool {
		return self.control.lock().unwrap().paused;
	}

	/// remove audio from mixer
	pub fn detach(&self) {
		self.control.lock().unwrap().detach = true;
	}

	pub fn add_effect(&self, e: Arc<Mutex<dyn Effect + Send>>) {
		self.mixer.lock().unwrap().add_effect(&self.id, e);
	}

}

