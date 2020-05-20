// wengwengweng

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::*;
use math::*;
use input::*;
use window::*;

pub struct Window {
	canvas: web_sys::HtmlCanvasElement,
	window: web_sys::Window,
	document: web_sys::Document,
	render_loop: Option<glow::RenderLoop>,
	pressed_keys: HashSet<Key>,
	pressed_mouse: HashSet<Mouse>,
	mouse_pos: Vec2,
	width: i32,
	height: i32,
	cursor_hidden: bool,
	prev_cursor: CursorIcon,
	title: String,
}

impl Window {

	pub(crate) fn new(conf: &conf::Conf) -> Result<Self> {

		let window = web_sys::window()
			.ok_or_else(|| format!("no window found"))?;

		let document = window
			.document()
			.ok_or_else(|| format!("should have a document on window"))?;

		document.set_title(&conf.title);

		let body = document
			.body()
			.ok_or_else(|| format!("no body found"))?;

		let canvas = match conf.canvas_mode {
			CanvasMode::Create => {
				document
					.create_element("canvas")
					.map_err(|_| format!("failed to create canvas"))?
					.dyn_into::<web_sys::HtmlCanvasElement>()
					.map_err(|_| format!("failed to create canvas"))?
			},
			CanvasMode::Get(id) => {
				document
					.query_selector(&format!("canvas#{}", id))
					.map_err(|_| format!("failed to create canvas"))?
					.ok_or_else(|| format!("failed to create canvas"))?
					.dyn_into::<web_sys::HtmlCanvasElement>()
					.map_err(|_| format!("failed to create canvas"))?
			},
		};

		canvas.set_width(conf.width as u32);
		canvas.set_height(conf.height as u32);
		canvas.set_attribute("alt", &conf.title);

		body
			.append_child(&canvas)
			.map_err(|_| format!("failed to append canvas"))?;

		let render_loop = glow::RenderLoop::from_request_animation_frame();

		return Ok(Self {
			window: window,
			document: document,
			canvas: canvas,
			render_loop: Some(render_loop),
			pressed_keys: hset![],
			pressed_mouse: hset![],
			mouse_pos: vec2!(),
			width: conf.width,
			height: conf.height,
			cursor_hidden: false,
			prev_cursor: CursorIcon::Normal,
			title: conf.title.to_string(),
		});

	}

}

impl Window {

	pub(crate) fn get_gl_ctx(&self) -> Result<gl::Device> {

		let webgl_context = self.canvas
			.get_context("webgl")
			.map_err(|_| format!("failed to fetch webgl context"))?
			.ok_or_else(|| format!("failed to fetch webgl context"))?
			.dyn_into::<web_sys::WebGlRenderingContext>()
			.map_err(|_| format!("failed to fetch webgl context"))?;

		return Ok(gl::Device::from_webgl_ctx(webgl_context));

	}

	pub(crate) fn swap(&self) -> Result<()> {
		return Ok(());
	}

	pub fn key_down(&self, k: Key) -> bool {
		return self.pressed_keys.contains(&k);
	}

	pub fn key_mods(&self) -> KeyMod {
		return KeyMod {
			shift: self.key_down(Key::LShift) || self.key_down(Key::RShift),
			ctrl: self.key_down(Key::LCtrl) || self.key_down(Key::RCtrl),
			alt: self.key_down(Key::LAlt) || self.key_down(Key::RAlt),
			meta: self.key_down(Key::LMeta) || self.key_down(Key::RMeta),
		};
	}

	pub fn mouse_down(&self, m: Mouse) -> bool {
		return self.pressed_mouse.contains(&m);
	}

	pub fn dpi(&self) -> f32 {
		return 1.0;
	}

	pub fn width(&self) -> i32 {
		return self.width;
	}

	pub fn height(&self) -> i32 {
		return self.height;
	}

	pub fn mouse_pos(&self) -> Vec2 {
		return self.mouse_pos;
	}

	pub fn set_mouse_pos(&mut self, _: Vec2) -> Result<()> {
		return Ok(());
	}

	pub fn set_fullscreen(&mut self, b: bool) {

		if b {
			self.canvas.request_fullscreen();
		} else {
			self.document.exit_fullscreen();
		}

	}

	pub fn is_fullscreen(&self) -> bool {
		return false;
	}

	pub fn set_cursor_hidden(&mut self, b: bool) {

		self.cursor_hidden = b;

		if b {
			self.canvas.set_attribute("style", "cursor: none");
		} else {
			self.canvas.set_attribute("style", &format!("cursor: {}", self.prev_cursor.to_web()));
		}

	}

	pub fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	pub fn set_cursor_locked(&mut self, b: bool) {
// 		if b {
// 			self.canvas.request_pointer_lock();
// 		} else {
// 			self.document.exit_pointer_lock();
// 		}
	}

	pub fn is_cursor_locked(&self) -> bool {
		return false;
	}

	pub fn set_title(&mut self, s: &str) {
		self.title = s.to_owned();
		self.document.set_title(s);
		self.canvas.set_attribute("alt", s);
	}

	pub fn title(&self) -> &str {
		return &self.title;
	}

	pub fn set_cursor(&mut self, c: CursorIcon) {
		self.prev_cursor = c;
		self.canvas.set_attribute("style", &format!("cursor: {}", c.to_web()));
	}

	pub fn quit(&mut self) {
		// ...
	}

	pub(crate) fn run(
		mut self,
		mut handle: impl FnMut(&mut Self, WindowEvent) -> Result<()> + 'static,
	) -> Result<()> {

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

				self.$root
					.add_event_listener_with_callback($name, handler.as_ref().unchecked_ref())
					.map_err(|_| format!("failed to add event {}", $name))?;

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
			None => return Ok(()),
		};

		render_loop.run(move |running: &mut bool| {

			let res: Result<()> = try {

				let mut events = vec![];

				for e in web_events.borrow().iter() {

					match e {

						WebEvent::KeyPress(e) => {
							if let Some(k) = Key::from_web(e) {
								events.push(KeyPressRepeat(k));
								if !self.key_down(k) {
									events.push(KeyPress(k));
								}
								self.pressed_keys.insert(k);
							}
						},

						WebEvent::KeyRelease(e) => {
							if let Some(k) = Key::from_web(e) {
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

						WebEvent::MousePress(_) => {
							self.pressed_mouse.insert(Mouse::Left);
							events.push(MousePress(Mouse::Left));
						},

						WebEvent::MouseRelease(_) => {
							self.pressed_mouse.remove(&Mouse::Left);
							events.push(MouseRelease(Mouse::Left));
						},

						WebEvent::Wheel(e) => {
							events.push(Wheel(vec2!(e.delta_x(), e.delta_y()), input::ScrollPhase::Solid));
						},

					}

				}

				web_events.borrow_mut().clear();

				for e in events {
					handle(&mut self, WindowEvent::Input(e))?;
				}

				handle(&mut self, WindowEvent::Frame)?;

			};

			if let Err(err) = res {
				elog!("{}", err);
			}

		});

		return Ok(());

	}

	pub fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	pub fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	pub fn toggle_cursor_locked(&mut self) {
		self.set_cursor_locked(!self.is_cursor_locked());
	}

}

impl CursorIcon {
	fn to_web(&self) -> &'static str {
		return match self {
			CursorIcon::Normal => "default",
			CursorIcon::Hand => "pointer",
			CursorIcon::Cross => "crosshair",
			CursorIcon::Move => "move",
			CursorIcon::Progress => "progress",
			CursorIcon::Wait => "wait",
			CursorIcon::Text => "text",
		};
	}
}

impl Key {

	fn from_web(e: &web_sys::KeyboardEvent) -> Option<Self> {

		return match e.code().as_ref() {
			"KeyQ" => Some(Key::Q),
			"KeyW" => Some(Key::W),
			"KeyE" => Some(Key::E),
			"KeyR" => Some(Key::R),
			"KeyT" => Some(Key::T),
			"KeyY" => Some(Key::Y),
			"KeyU" => Some(Key::U),
			"KeyI" => Some(Key::I),
			"KeyO" => Some(Key::O),
			"KeyP" => Some(Key::P),
			"KeyA" => Some(Key::A),
			"KeyS" => Some(Key::S),
			"KeyD" => Some(Key::D),
			"KeyF" => Some(Key::F),
			"KeyG" => Some(Key::G),
			"KeyH" => Some(Key::H),
			"KeyJ" => Some(Key::J),
			"KeyK" => Some(Key::K),
			"KeyL" => Some(Key::L),
			"KeyZ" => Some(Key::Z),
			"KeyX" => Some(Key::X),
			"KeyC" => Some(Key::C),
			"KeyV" => Some(Key::V),
			"KeyB" => Some(Key::B),
			"KeyN" => Some(Key::N),
			"KeyM" => Some(Key::M),
			"Digit1" => Some(Key::Key1),
			"Digit2" => Some(Key::Key2),
			"Digit3" => Some(Key::Key3),
			"Digit4" => Some(Key::Key4),
			"Digit5" => Some(Key::Key5),
			"Digit6" => Some(Key::Key6),
			"Digit7" => Some(Key::Key7),
			"Digit8" => Some(Key::Key8),
			"Digit9" => Some(Key::Key9),
			"Digit0" => Some(Key::Key0),
			"F1" => Some(Key::F1),
			"F2" => Some(Key::F2),
			"F3" => Some(Key::F3),
			"F4" => Some(Key::F4),
			"F5" => Some(Key::F5),
			"F6" => Some(Key::F6),
			"F7" => Some(Key::F7),
			"F8" => Some(Key::F8),
			"F9" => Some(Key::F9),
			"F10" => Some(Key::F10),
			"F11" => Some(Key::F11),
			"F12" => Some(Key::F12),
			"Minus" => Some(Key::Minus),
			"Equal" => Some(Key::Equal),
			"Comma" => Some(Key::Comma),
			"Period" => Some(Key::Period),
			"Backquote" => Some(Key::Backquote),
			"Slash" => Some(Key::Slash),
			"Backslash" => Some(Key::Backslash),
			"Semicolon" => Some(Key::Semicolon),
			"Quote" => Some(Key::Quote),
			"ArrowUp" => Some(Key::Up),
			"ArrowDown" => Some(Key::Down),
			"ArrowLeft" => Some(Key::Left),
			"ArrowRight" => Some(Key::Right),
			"Escape" => Some(Key::Esc),
			"Tab" => Some(Key::Tab),
			"Space" => Some(Key::Space),
			"Backspace" => Some(Key::Backspace),
			"Enter" => Some(Key::Enter),
			"ShiftLeft" => Some(Key::LShift),
			"ShiftRight" => Some(Key::RShift),
			"AltLeft" => Some(Key::LAlt),
			"AltRight" => Some(Key::RAlt),
			"MetaLeft" => Some(Key::LMeta),
			"MetaRight" => Some(Key::RMeta),
			"ControlLeft" => Some(Key::LCtrl),
			"ControlRight" => Some(Key::RCtrl),
			_ => None,

		};

	}

}

