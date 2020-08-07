// wengwengweng

use super::*;

/// Streamed Sound (mainly for music)
#[derive(Clone)]
pub struct Track {
// 	id: SourceID,
	src: Arc<Mutex<Decoder<Cursor<Vec<u8>>>>>,
	ctrl: Arc<Mutex<Control>>,
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

		let ctrl = mixer.add(src.clone());

		ctrl.lock().unwrap().paused = true;

		return Ok(Self {
			src: src,
			ctrl: ctrl,
			mixer: ctx.mixer().clone(),
		});

	}

	/// play / resume track
	pub fn play(&self) {
		self.ctrl.lock().unwrap().paused = false;
	}

	/// pause track
	pub fn pause(&self) {
		self.ctrl.lock().unwrap().paused = true;
	}

	/// set volume
	pub fn set_volume(&self, v: f32) {
		self.ctrl.lock().unwrap().volume = v;
	}

	/// get volume
	pub fn volume(&self) -> f32 {
		return self.ctrl.lock().unwrap().volume;
	}

	/// set pan
	pub fn set_pan(&self, l: f32, r: f32) {
		self.ctrl.lock().unwrap().pan = Pan::new(l, r);
	}

	/// get pan
	pub fn pan(&self) -> Pan {
		return self.ctrl.lock().unwrap().pan;
	}

	/// set looping
	pub fn set_looping(&self, l: bool) {
		self.ctrl.lock().unwrap().looping = l;
	}

	/// check if is paused
	pub fn paused(&self) -> bool {
		return self.ctrl.lock().unwrap().paused;
	}

	/// remove audio from mixer
	pub fn detach(&self) {
		self.ctrl.lock().unwrap().detach = true;
	}

	pub fn add_effect(&self, e: Arc<Mutex<dyn Effect + Send>>) {
		self.ctrl.lock().unwrap().effects.push(e);
	}

}

