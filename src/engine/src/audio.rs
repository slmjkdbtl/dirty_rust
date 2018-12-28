// wengwengweng

static mut AUDIO: Option<AudioCtx> = None;

fn get_ctx() -> &'static AudioCtx {

	unsafe {
		match &AUDIO {
			Some(g) => {
				return g;
			}
			None => {
				panic!("audio not initialized");
			},
		}
	}

}

fn get_ctx_mut() -> &'static mut AudioCtx {

	unsafe {
		match &mut AUDIO {
			Some(g) => {
				return g;
			}
			None => {
				panic!("gfx not initialized");
			},
		}
	}

}

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

