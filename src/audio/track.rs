// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::io::Cursor;

use super::*;

/// Streamed Sound (mainly for music)
#[derive(Clone)]
pub struct Track {
	src: Arc<Mutex<dyn Source + Send>>,
	ctrl: Arc<Mutex<Control>>,
}

impl Track {

	/// create track from bytes of an audio file
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let src = Decoder::new(Cursor::new(data.to_owned()))?;
		let src = Arc::new(Mutex::new(src));
		let ctrl = Arc::new(Mutex::new(Control {
			pan: 0.0,
			paused: true,
			volume: 1.0,
		}));

		let t = Self {
			src: src,
			ctrl: ctrl,
		};

		if let Ok(mut mixer) = ctx.mixer().lock() {
			mixer.add_with_ctrl(Arc::clone(&t.src), Arc::clone(&t.ctrl));
		}

		return Ok(t);

	}

	/// play / resume track
	pub fn play(&self) {
		if let Ok(mut ctrl) = self.ctrl.lock() {
			ctrl.paused = false;
		}
	}

	/// pause track
	pub fn pause(&self) {
		if let Ok(mut ctrl) = self.ctrl.lock() {
			ctrl.paused = true;
		}
	}

	/// set pan
	pub fn set_pan(&self, pan: f32) {
		if let Ok(mut ctrl) = self.ctrl.lock() {
			ctrl.pan = pan;
		}
	}

	/// set volume
	pub fn set_volume(&self, v: f32) {
		if let Ok(mut ctrl) = self.ctrl.lock() {
			ctrl.volume = v;
		}
	}

	/// check if is paused
	pub fn paused(&self) -> bool {
		if let Ok(ctrl) = self.ctrl.lock() {
			return ctrl.paused;
		}
		// TODO
		return true;
	}

}

