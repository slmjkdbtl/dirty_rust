// wengwengweng

use crate::ctx;

ctx!(AUDIO: AudioCtx);

struct AudioCtx {
	device: rodio::Device,
}

pub fn init() {

	let device = rodio::default_output_device().unwrap();

	init_ctx(AudioCtx {
		device: device,
	});

}

