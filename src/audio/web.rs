// wengwengweng

use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::*;

pub struct Audio {
	ctx: web_sys::AudioContext,
}

impl Audio {
	pub fn new() -> Result<Self> {
		let ctx = web_sys::AudioContext::new().map_err(|_| format!("failed to create audio context"))?;
		return Ok(Self {
			ctx: ctx,
		});
	}
}

pub struct Sound {
	src: web_sys::AudioBufferSourceNode,
}

impl Sound {

	pub fn play(&self) {
		log!("{:?}", self.src.start());
	}

	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let buf = js_sys::Uint8Array::from(data);
		let src = ctx.ctx
			.create_buffer_source()
			.map_err(|_| format!("failed to create audio source"))?;

		let src2 = src.clone();
		let dest = ctx.ctx.destination();

		let handler = Closure::wrap(box (move |b: web_sys::AudioBuffer| {
			src2.set_buffer(Some(&b));
			src2.connect_with_audio_node(&dest);
			src2.set_loop(true);
		}) as Box<dyn FnMut(_)>);

		ctx.ctx
			.decode_audio_data_with_success_callback(&buf.buffer(), handler.as_ref().unchecked_ref())
			.map_err(|_| format!("failed to decode audio"))?;

		handler.forget();

		return Ok(Self {
			src: src,
		});

	}

}

