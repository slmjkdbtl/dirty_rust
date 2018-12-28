// wengwengweng

use crate::utils;
use crate::create_context;

create_context!(AUDIO, AudioCtx);

struct AudioCtx {
	device: rodio::Device,
}

pub fn init() {

	let device = rodio::default_output_device().unwrap();

	unsafe {
		AUDIO = Some(AudioCtx {
			device: device,
		});
	}

}

