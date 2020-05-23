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

	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let src = Decoder::new(Cursor::new(data.to_owned()))?;
		let src = Arc::new(Mutex::new(src));
		let mut ctrl = Control::new();
		ctrl.paused = true;

		let ctrl = Arc::new(Mutex::new(ctrl));

		let t = Self {
			src: src,
			ctrl: ctrl,
		};

		if let Ok(mut mixer) = ctx.mixer().lock() {
			mixer.add_with_ctrl(Arc::clone(&t.src), Arc::clone(&t.ctrl));
		}

		return Ok(t);

	}

	pub fn play(&self) {
		if let Ok(mut ctrl) = self.ctrl.lock() {
			ctrl.paused = false;
		}
	}

	pub fn pause(&self) {
		if let Ok(mut ctrl) = self.ctrl.lock() {
			ctrl.paused = true;
		}
	}

	pub fn paused(&self) -> bool {
		if let Ok(ctrl) = self.ctrl.lock() {
			return ctrl.paused;
		}
		// TODO
		return true;
	}

}

