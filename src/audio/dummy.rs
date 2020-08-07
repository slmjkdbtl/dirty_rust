// wengwengweng

use super::*;

pub struct Audio {
	mixer: Arc<Mutex<Mixer>>,
}

impl Audio {
	pub(crate) fn new(_: &conf::Conf) -> Result<Self> {
		return Ok(Self {
			mixer: Arc::new(Mutex::new(Mixer::new(SPEC))),
		});
	}
	pub(super) fn mixer(&self) -> &Arc<Mutex<Mixer>> {
		return &self.mixer;
	}
}

