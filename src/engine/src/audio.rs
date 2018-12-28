// wengwengweng

use crate::create_ctx;
use crate::init_ctx;

create_ctx!(AUDIO: AudioCtx);

struct AudioCtx {
	device: rodio::Device,
}

pub fn init() {

	let device = rodio::default_output_device().unwrap();

	init_ctx!(AUDIO -> AudioCtx {
		device: device,
	});

}

