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
		let ctx = web_sys::AudioContext::new()
			.map_err(|_| format!("failed to create audio context"))?;
		return Ok(Self {
			ctx: ctx,
		});
	}
}

struct Decoder {
	buffer: Rc<RefCell<Option<web_sys::AudioBuffer>>>,
}

impl Decoder {

	fn new(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let buf = js_sys::Uint8Array::from(data);
		let abuf = Rc::new(RefCell::new(None));
		let abuf2 = abuf.clone();

		let handler = Closure::wrap(box (move |b: web_sys::AudioBuffer| {
			*abuf2.borrow_mut() = Some(b);
		}) as Box<dyn FnMut(_)>);

		ctx.ctx
			.decode_audio_data_with_success_callback(&buf.buffer(), handler.as_ref().unchecked_ref())
			.map_err(|_| format!("failed to decode audio"))?;

		handler.forget();

		return Ok(Self {
			buffer: abuf,
		});

	}

	fn buffer(&self) -> std::cell::Ref<Option<web_sys::AudioBuffer>> {
		return self.buffer.borrow();
	}

}

pub struct Sound {
	decoder: Decoder,
}

impl Sound {

	pub fn play(&self, ctx: &Audio) -> Result<()> {

		let src = ctx.ctx
			.create_buffer_source()
			.map_err(|_| format!("failed to create audio source"))?;

		src.connect_with_audio_node(&ctx.ctx.destination());
		src.set_buffer(self.decoder.buffer().as_ref());
		src.start();

		return Ok(());

	}

	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {
		return Ok(Self {
			decoder: Decoder::new(ctx, data)?,
		});
	}

}

pub struct Track {
	decoder: Decoder,
}

impl Track {
	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {
		return Ok(Self {
			decoder: Decoder::new(ctx, data)?,
		});
	}
	pub fn play(&self, ctx: &Audio) {
		// ..
	}
	pub fn pause(&self, ctx: &Audio) {
		// ..
	}
	pub fn is_playing(&self) -> bool {
		return false;
	}
}

