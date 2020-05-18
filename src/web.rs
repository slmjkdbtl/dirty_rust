// wengwengweng

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

use crate::*;
use math::*;
use input::*;
use window::WindowEvent;

pub struct Window {
	canvas: web_sys::HtmlCanvasElement,
	window: web_sys::Window,
	document: web_sys::Document,
	render_loop: Option<glow::RenderLoop>,
	gl: Rc<gl::Device>,
	pressed_keys: HashSet<Key>,
	pressed_mouse: HashSet<Mouse>,
	mouse_pos: Vec2,
	width: i32,
	height: i32,
}

impl Window {

	pub fn new(conf: &conf::Conf) -> Result<Self> {

		use wasm_bindgen::JsCast;

		let window = web_sys::window()
			.ok_or_else(|| format!("no window found"))?;

		let document = window
			.document()
			.ok_or_else(|| format!("should have a document on window"))?;

		document.set_title(&conf.title);

		let body = document
			.body()
			.ok_or_else(|| format!("no body found"))?;

		let canvas = document
			.create_element("canvas")
			.map_err(|_| format!("failed to create canvas"))?
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.map_err(|_| format!("failed to create canvas"))?;

		canvas.set_width(conf.width as u32);
		canvas.set_height(conf.height as u32);

		let webgl_context = canvas
			.get_context("webgl")
			.map_err(|_| format!("failed to fetch webgl context"))?
			.ok_or_else(|| format!("failed to fetch webgl context"))?
			.dyn_into::<web_sys::WebGlRenderingContext>()
			.map_err(|_| format!("failed to fetch webgl context"))?;

		body
			.append_child(&canvas)
			.map_err(|_| format!("failed to append canvas"))?;

		let gl = gl::Device::from_webgl_ctx(webgl_context);
		let render_loop = glow::RenderLoop::from_request_animation_frame();

		return Ok(Self {
			window: window,
			document: document,
			canvas: canvas,
			gl: Rc::new(gl),
			render_loop: Some(render_loop),
			pressed_keys: hset![],
			pressed_mouse: hset![],
			mouse_pos: vec2!(),
			width: conf.width,
			height: conf.height,
		});

	}

}

impl window::WindowCtx for Window {

	fn gl(&self) -> &gl::Device {
		return &self.gl;
	}

	fn swap(&self) -> Result<()> {
		return Ok(());
	}

	fn key_down(&self, k: Key) -> bool {
		return self.pressed_keys.contains(&k);
	}

	fn mouse_down(&self, m: Mouse) -> bool {
		return self.pressed_mouse.contains(&m);
	}

	fn dpi(&self) -> f32 {
		return 1.0;
	}

	fn width(&self) -> i32 {
		return self.width;
	}

	fn height(&self) -> i32 {
		return self.height;
	}

	fn run(
		mut self,
		mut handle: impl FnMut(&mut Self, WindowEvent) -> Result<()> + 'static,
	) {

		use wasm_bindgen::JsCast;
		use wasm_bindgen::closure::Closure;
		use std::cell::RefCell;
		use input::Event::*;

		let web_events = Rc::new(RefCell::new(vec![]));

		enum WebEvent {
			KeyPress(web_sys::KeyboardEvent),
			KeyRelease(web_sys::KeyboardEvent),
			MouseMove(web_sys::MouseEvent),
			MousePress(web_sys::MouseEvent),
			MouseRelease(web_sys::MouseEvent),
			Wheel(web_sys::WheelEvent),
		}

		macro_rules! add_event {

			($root:ident, $name:expr, $ty:ty, $t:ident) => {

				let web_events_c = web_events.clone();

				let handler = Closure::wrap(box (move |e: $ty| {
					web_events_c.borrow_mut().push(WebEvent::$t(e));
				}) as Box<dyn FnMut(_)>);

				self.$root.add_event_listener_with_callback($name, handler.as_ref().unchecked_ref());

				handler.forget();

			}

		}

		add_event!(document, "keydown", web_sys::KeyboardEvent, KeyPress);
		add_event!(document, "keyup", web_sys::KeyboardEvent, KeyRelease);
		add_event!(canvas, "mousemove", web_sys::MouseEvent, MouseMove);
		add_event!(canvas, "mousedown", web_sys::MouseEvent, MousePress);
		add_event!(canvas, "mouseup", web_sys::MouseEvent, MouseRelease);
		add_event!(canvas, "wheel", web_sys::WheelEvent, Wheel);

		use glow::HasRenderLoop;

		let render_loop = match self.render_loop.take() {
			Some(l) => l,
			None => return,
		};

		render_loop.run(move |running: &mut bool| {

			let mut events = vec![];

			for e in web_events.borrow().iter() {

				match e {

					WebEvent::KeyPress(e) => {
						if let Some(k) = Key::from_code(e.key_code()) {
							events.push(KeyPressRepeat(k));
							if !self.key_down(k) {
								events.push(KeyPress(k));
							}
							self.pressed_keys.insert(k);
						}
					},

					WebEvent::KeyRelease(e) => {
						if let Some(k) = Key::from_code(e.key_code()) {
							self.pressed_keys.remove(&k);
							events.push(KeyRelease(k));
						}
					},

					WebEvent::MouseMove(e) => {

						let (w, h) = (self.width as f32, self.height as f32);
						let mpos = vec2!(e.client_x(), e.client_y());
						let mpos = vec2!(mpos.x - w / 2.0, h / 2.0 - mpos.y as f32);
						let prev_mpos = self.mouse_pos;

						self.mouse_pos = mpos;

						if prev_mpos != vec2!(0) {
							events.push(MouseMove(mpos - prev_mpos));
						}

					},

					WebEvent::MousePress(e) => {
						self.pressed_mouse.insert(Mouse::Left);
						events.push(MousePress(Mouse::Left));
					},

					WebEvent::MouseRelease(e) => {
						self.pressed_mouse.remove(&Mouse::Left);
						events.push(MouseRelease(Mouse::Left));
					},

					WebEvent::Wheel(e) => {
						events.push(Wheel(vec2!(e.delta_x(), e.delta_y()), input::ScrollPhase::Solid));
					},

					_ => {},

				}

			}

			web_events.borrow_mut().clear();

			for e in events {
				handle(&mut self, WindowEvent::Input(e));
			}

			handle(&mut self, WindowEvent::Frame);

		});

	}

}

