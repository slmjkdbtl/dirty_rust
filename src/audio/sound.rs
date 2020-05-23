// wengwengweng

use std::sync::Mutex;
use std::sync::Arc;
use std::io::Cursor;

use super::*;

#[derive(Clone)]
pub struct Sound {
	buf: Buffered,
	mixer: Arc<Mutex<Mixer>>,
}

impl Sound {

	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let buf = Buffered::from_source(VorbisDecoder::from_reader(Cursor::new(data.to_owned()))?);

		let t = Self {
			buf: buf,
			mixer: Arc::clone(ctx.mixer()),
		};

		return Ok(t);

	}

	pub fn play(&self) {
		if let Ok(mut mixer) = self.mixer.lock() {
			mixer.add(Arc::new(Mutex::new(self.buf.clone())));
		}
	}

}

