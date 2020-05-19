// wengwengweng

use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::*;

pub struct Audio {
	ctx: Rc<web_sys::AudioContext>,
}

impl Audio {
	pub fn new() -> Result<Self> {
		let ctx = web_sys::AudioContext::new()
			.map_err(|_| format!("failed to create audio context"))?;
		return Ok(Self {
			ctx: Rc::new(ctx),
		});
	}
}

pub struct Sound {
	buffer: Rc<RefCell<Option<web_sys::AudioBuffer>>>,
	ctx: Rc<web_sys::AudioContext>,
}

impl Sound {

	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

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
			ctx: ctx.ctx.clone(),
		});

	}

	pub fn play(&self) -> Result<()> {

		let src = self.ctx
			.create_buffer_source()
			.map_err(|_| format!("failed to create audio source"))?;

		src.connect_with_audio_node(&self.ctx.destination());
		src.set_buffer(self.buffer.borrow().as_ref());
		src.start();

		return Ok(());

	}

}

pub struct Track {
	audio: web_sys::HtmlAudioElement,
}

impl Track {

	pub fn from_bytes(ctx: &Audio, data: &[u8]) -> Result<Self> {

		let buffer = js_sys::Uint8Array::from(data);
		let buffer_val: &wasm_bindgen::JsValue = buffer.as_ref();
		let parts = js_sys::Array::new_with_length(1);

		parts.set(0, buffer_val.clone());

		let blob = web_sys::Blob::new_with_u8_array_sequence(parts.as_ref())
			.map_err(|_| format!("failed to create track"))?;

		let src = web_sys::Url::create_object_url_with_blob(&blob)
			.map_err(|_| format!("failed to create track"))?;

		let audio = web_sys::HtmlAudioElement::new_with_src(&src)
			.map_err(|_| format!("failed to create audio element"))?;

		return Ok(Self {
			audio: audio,
		});

	}

	pub fn play(&self) {
		self.audio.play();
	}

	pub fn pause(&self) {
		self.audio.pause();
	}

	pub fn is_playing(&self) -> bool {
		return !self.audio.paused();
	}
}

