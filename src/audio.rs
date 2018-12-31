// wengwengweng

use crate::*;

// context
ctx!(AUDIO: AudioCtx);

struct AudioCtx {
	device: rodio::Device,
}

// local public functions
pub(crate) fn init() {

	let device = rodio::default_output_device().unwrap();

	init_ctx(AudioCtx {
		device: device,
	});

}

