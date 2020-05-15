// wengwengweng

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

macro_rules! console_log {
	($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn yo() {
	if let Err(e) = run() {
		console_log!("{}", e);
	}
}

fn run() -> Result<(), String> {

	let window = web_sys::window()
		.ok_or_else(|| format!("no window found"))?;

	let document = window
		.document()
		.ok_or_else(|| format!("should have a document on window"))?;

	let body = document
		.body()
		.ok_or_else(|| format!("no body found"))?;

	let canvas = document
		.create_element("canvas")
		.map_err(|_| format!("failed to create canvas"))?
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.map_err(|_| format!("failed to create canvas"))?;

	let webgl_context = canvas
		.get_context("webgl")
		.map_err(|_| format!("failed to fetch webgl context"))?
		.ok_or_else(|| format!("failed to fetch webgl context"))?
		.dyn_into::<web_sys::WebGlRenderingContext>()
		.map_err(|_| format!("failed to fetch webgl context"))?;

	body
		.append_child(&canvas)
		.map_err(|_| format!("failed to append canvas"))?;

	return Ok(());

}

