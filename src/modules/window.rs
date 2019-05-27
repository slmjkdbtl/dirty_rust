// wengwengweng

//! Window & Graphics

use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use std::time::Duration;

use glutin::dpi::*;
use glutin::Api;
use glutin::GlRequest;
use glutin::ElementState;
use derive_more::*;
use gilrs::Gilrs;
use serde::Serialize;
use serde::Deserialize;

pub use glutin::ModifiersState as Mod;
pub use glutin::VirtualKeyCode as Key;
pub use glutin::MouseButton as Mouse;

use crate::math::*;
use crate::*;

pub struct Window {
	conf: Conf,
	ctx: Ctx,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

#[derive(Copy, Clone, PartialEq, Debug, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, From, Into)]
pub struct MousePos {
	pub x: i32,
	pub y: i32,
}

impl MousePos {
	fn new(x: i32, y: i32) -> Self {
		return Self {
			x: x,
			y: y,
		};
	}
}

impl From<MousePos> for Vec2 {
	fn from(mpos: MousePos) -> Self {
		return vec2!(mpos.x, mpos.y);
	}
}

impl From<LogicalPosition> for MousePos {
	fn from(pos: LogicalPosition) -> Self {
		let (x, y): (i32, i32) = pos.into();
		return Self {
			x: x,
			y: y,
		};
	}
}

impl From<MouseDelta> for MousePos {
	fn from(pos: MouseDelta) -> Self {
		return Self {
			x: pos.x,
			y: pos.y,
		};
	}
}

impl From<MousePos> for MouseDelta {
	fn from(pos: MousePos) -> Self {
		return Self {
			x: pos.x,
			y: pos.y,
		};
	}
}

impl From<MousePos> for LogicalPosition {
	fn from(pos: MousePos) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

#[derive(Copy, Clone, PartialEq, Debug, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, From, Into)]
pub struct MouseDelta {
	pub x: i32,
	pub y: i32,
}

impl MouseDelta {
	pub fn new(x: i32, y: i32) -> Self {
		return Self {
			x: x,
			y: y,
		};
	}
	pub fn is_none(&self) -> bool {
		return self.x == 0 && self.y == 0;
	}
}

impl From<MouseDelta> for Vec2 {
	fn from(mpos: MouseDelta) -> Self {
		return vec2!(mpos.x, mpos.y);
	}
}

impl From<LogicalPosition> for MouseDelta {
	fn from(pos: LogicalPosition) -> Self {
		let (x, y): (i32, i32) = pos.into();
		return Self {
			x: x,
			y: y,
		};
	}
}

impl From<LogicalSize> for Size {
	fn from(size: LogicalSize) -> Self {
		return Self {
			w: size.width as u32,
			h: size.height as u32,
		};
	}
}

#[derive(Copy, Clone, PartialEq, Debug, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, From, Into)]
pub struct ScrollDelta {
	pub x: i32,
	pub y: i32,
}

impl ScrollDelta {

	pub fn new(x: i32, y: i32) -> Self {
		return Self {
			x: x,
			y: y,
		};
	}

}

impl From<ScrollDelta> for Vec2 {
	fn from(sdis: ScrollDelta) -> Self {
		return vec2!(sdis.x, sdis.y);
	}
}

impl From<glutin::MouseScrollDelta> for ScrollDelta {
	fn from(delta: glutin::MouseScrollDelta) -> Self {
		use glutin::MouseScrollDelta;
		match delta {
			MouseScrollDelta::PixelDelta(pos) => {
				let (x, y): (i32, i32) = pos.into();
				return Self {
					x: x,
					y: y,
				};
			},
			MouseScrollDelta::LineDelta(x, y) => {
				return Self {
					x: x as i32,
					y: y as i32,
				};
			}
		};
	}
}

impl From<Vec2> for LogicalPosition {
	fn from(pos: Vec2) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

impl From<LogicalPosition> for Vec2 {
	fn from(pos: LogicalPosition) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conf {
	pub width: u32,
	pub height: u32,
	pub title: &'static str,
	pub hidpi: bool,
	pub resizable: bool,
	pub fullscreen: bool,
	pub always_on_top: bool,
	pub multi_touch: bool,
	pub borderless: bool,
	pub transparent: bool,
	pub vsync: bool,
	pub hide_title: bool,
	pub hide_titlebar_buttons: bool,
	pub fullsize_content: bool,
	pub titlebar_transparent: bool,
	pub cursor_hidden: bool,
	pub cursor_locked: bool,
}

impl Conf {

	pub fn basic(title: &'static str, width: u32, height: u32) -> Self {
		return Self {
			title: title,
			width: width,
			height: height,
			..Default::default()
		};
	}

}

impl Default for Conf {

	fn default() -> Self {
		return Self {
			width: 640,
			height: 480,
			title: "",
			hidpi: true,
			resizable: false,
			fullscreen: false,
			always_on_top: false,
			multi_touch: false,
			borderless: false,
			transparent: false,
			vsync: false,
			fullsize_content: false,
			hide_title: false,
			hide_titlebar_buttons: false,
			titlebar_transparent: false,
			cursor_hidden: false,
			cursor_locked: false,
		};
	}

}

impl Default for Window {
	fn default() -> Self {
		return Self::new(Conf::default());
	}
}

#[derive(Clone)]
enum WindowRequest {
	Fullscreen(bool),
	HideCursor(bool),
	LockCursor(bool),
	SetTitle(String),
}

#[derive(Clone)]
pub struct Ctx {
	dt: f32,
	time: f32,
	closed: bool,
	fps_cap: u32,
	key_state: HashMap<Key, ButtonState>,
	mouse_state: HashMap<Mouse, ButtonState>,
	mouse_pos: MousePos,
	mouse_delta: Option<MouseDelta>,
	scroll_delta: Option<ScrollDelta>,
	text_input: Option<String>,
	window_requests: Vec<WindowRequest>,
	title: String,
	fullscreen: bool,
	cursor_hidden: bool,
	cursor_locked: bool,
}

impl Ctx {

	/// get delta time between frames
	pub fn dt(&self) -> f32 {
		return self.dt;
	}

	/// get current framerate
	pub fn fps(&self) -> u32 {
		return (1.0 / self.dt) as u32;
	}

	/// get actual time since running
	pub fn time(&self) -> f32 {
		return self.time;
	}

	pub fn close(&mut self) {
		self.closed = true;
	}

	pub fn down_keys(&self) -> HashSet<Key> {

		use ButtonState::*;

		return self.key_state
			.iter()
			.filter(|(_, &state)| state == Down || state == Pressed)
			.map(|(key, _)| *key)
			.collect();

	}

	pub fn key_down(&self, key: Key) -> bool {
		return self.key_state.get(&key) == Some(&ButtonState::Down) || self.key_pressed(key);
	}

	pub fn key_pressed(&self, key: Key) -> bool {
		return self.key_state.get(&key) == Some(&ButtonState::Pressed);
	}

	pub fn key_released(&self, key: Key) -> bool {
		return self.key_state.get(&key) == Some(&ButtonState::Released);
	}

	pub fn key_up(&self, key: Key) -> bool {
		return self.key_state.get(&key) == Some(&ButtonState::Up) || self.key_state.get(&key).is_none();
	}

	pub fn mouse_down(&self, mouse: Mouse) -> bool {
		return self.mouse_state.get(&mouse) == Some(&ButtonState::Down) || self.mouse_pressed(mouse);
	}

	pub fn mouse_pressed(&self, mouse: Mouse) -> bool {
		return self.mouse_state.get(&mouse) == Some(&ButtonState::Pressed);
	}

	pub fn mouse_released(&self, mouse: Mouse) -> bool {
		return self.mouse_state.get(&mouse) == Some(&ButtonState::Released);
	}

	pub fn mouse_up(&self, mouse: Mouse) -> bool {
		return self.mouse_state.get(&mouse) == Some(&ButtonState::Up) || self.mouse_state.get(&mouse).is_none();
	}

	pub fn mouse_pos(&self) -> MousePos {
		return self.mouse_pos;
	}

	pub fn mouse_delta(&self) -> Option<MouseDelta> {
		return self.mouse_delta;
	}

	pub fn scroll_delta(&self) -> Option<ScrollDelta> {
		return self.scroll_delta;
	}

	pub fn text_input(&self) -> Option<String> {
		return self.text_input.clone();
	}

	pub fn set_fullscreen(&mut self, b: bool) {
		self.window_requests.push(WindowRequest::Fullscreen(b));
	}

	pub fn is_fullscreen(&self) -> bool {
		return self.fullscreen;
	}

	pub fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	pub fn set_cursor_hidden(&mut self, b: bool) {
		self.window_requests.push(WindowRequest::HideCursor(b));
	}

	pub fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	pub fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	pub fn set_cursor_locked(&mut self, b: bool) {
		self.window_requests.push(WindowRequest::LockCursor(b));
	}

	pub fn is_cursor_locked(&self) -> bool {
		return self.cursor_locked;
	}

	pub fn toggle_cursor_locked(&mut self) {
		self.set_cursor_locked(!self.is_cursor_locked());
	}

	pub fn set_title(&mut self, t: &str) {
		self.window_requests.push(WindowRequest::SetTitle(t.to_owned()));
	}

	pub fn title(&self) -> String {
		return self.title.clone();
	}

}

impl Window {

	pub fn new(conf: Conf) -> Self {

		let ctx = Ctx {
			dt: 0.0,
			time: 0.0,
			closed: false,
			fps_cap: 60,
			key_state: HashMap::new(),
			mouse_state: HashMap::new(),
			mouse_pos: MousePos::new(0, 0),
			mouse_delta: None,
			scroll_delta: None,
			text_input: None,
			window_requests: Vec::new(),
			fullscreen: conf.fullscreen,
			cursor_hidden: conf.cursor_hidden,
			cursor_locked: conf.cursor_locked,
			title: conf.title.to_owned(),
		};

		return Self {
			conf: conf,
			ctx: ctx,
		};

	}

	pub fn ctx(&mut self) -> &mut Ctx {
		return &mut self.ctx;
	}

	pub fn run(&mut self, mut f: impl FnMut(&mut Ctx)) -> Result<()> {

		let mut event_loop = glutin::EventsLoop::new();
		let monitor = event_loop.get_primary_monitor();

		let mut window_builder = glutin::WindowBuilder::new()
			.with_title(self.conf.title.to_owned())
			.with_resizable(self.conf.resizable)
			.with_transparency(self.conf.transparent)
			.with_decorations(!self.conf.borderless)
			.with_always_on_top(self.conf.always_on_top)
			.with_dimensions(LogicalSize::new(self.conf.width as f64, self.conf.height as f64));

		if self.conf.fullscreen {
			window_builder = window_builder
				.with_fullscreen(Some(monitor));
		}

		if self.conf.multi_touch {
			window_builder = window_builder
				.with_multitouch();
		}

		#[cfg(target_os = "macos")] {

			use glutin::os::macos::WindowBuilderExt;

			window_builder = window_builder
				.with_titlebar_buttons_hidden(self.conf.hide_titlebar_buttons)
				.with_title_hidden(self.conf.hide_title)
				.with_titlebar_transparent(self.conf.titlebar_transparent)
				.with_fullsize_content_view(self.conf.fullsize_content);

		}

		let ctx_builder = glutin::ContextBuilder::new()
			.with_vsync(self.conf.vsync)
			.with_gl(GlRequest::Specific(Api::OpenGl, (2, 1)));

		let windowed_ctx = ctx_builder
			.build_windowed(window_builder, &event_loop)?;

		let windowed_ctx = unsafe {
			let windowed_ctx = windowed_ctx.make_current()?;
			gl::load_with(|symbol| windowed_ctx.get_proc_address(symbol) as *const _);
			windowed_ctx
		};

		let window = windowed_ctx.window();

		let mut gamepad_ctx = Gilrs::new()?;

		ggl::clear(true, false, false);
		windowed_ctx.swap_buffers()?;

		loop {

			let start_time = Instant::now();

			for state in self.ctx.key_state.values_mut() {
				if state == &ButtonState::Pressed {
					*state = ButtonState::Down;
				} else if state == &ButtonState::Released {
					*state = ButtonState::Up;
				}
			}

			for state in self.ctx.mouse_state.values_mut() {
				if state == &ButtonState::Pressed {
					*state = ButtonState::Down;
				} else if state == &ButtonState::Released {
					*state = ButtonState::Up;
				}
			}

			self.ctx.mouse_delta = None;
			self.ctx.scroll_delta = None;
			self.ctx.text_input = None;

			event_loop.poll_events(|e| {

				use glutin::Event::*;
				use glutin::WindowEvent::*;

				match e {

					WindowEvent { event, .. } => match event {

						KeyboardInput { input, .. } => {
							if let Some(kc) = input.virtual_keycode {
								match input.state {
									ElementState::Pressed => {
										if self.ctx.key_up(kc) || self.ctx.key_released(kc) {
											self.ctx.key_state.insert(kc, ButtonState::Pressed);
										}
									},
									ElementState::Released => {
										if self.ctx.key_down(kc) || self.ctx.key_pressed(kc) {
											self.ctx.key_state.insert(kc, ButtonState::Released);
										}
									},
								}
							}
						},

						MouseInput { button, state, .. } => {
							match state {
								ElementState::Pressed => {
									if self.ctx.mouse_up(button) || self.ctx.mouse_released(button) {
										self.ctx.mouse_state.insert(button, ButtonState::Pressed);
									}
								},
								ElementState::Released => {
									if self.ctx.mouse_down(button) || self.ctx.mouse_pressed(button) {
										self.ctx.mouse_state.insert(button, ButtonState::Released);
									}
								},
							}
						},

						CursorMoved { position, .. } => {

							let pos: MousePos = position.into();

							self.ctx.mouse_delta = Some((pos - self.ctx.mouse_pos).into());
							self.ctx.mouse_pos = pos;

						},

						MouseWheel { delta, .. } => {
							self.ctx.scroll_delta = Some(delta.into());
						},

						ReceivedCharacter(ch) => {
							self.ctx.text_input.get_or_insert(String::new()).push(ch);
						},

						CloseRequested => self.ctx.close(),

						_ => {},

					},

					_ => {},

				};

			});

			while let Some(gilrs::Event { id, event, .. }) = gamepad_ctx.next_event() {
				// ...
			}

			ggl::clear(true, false, false);
			f(&mut self.ctx);
			windowed_ctx.swap_buffers()?;

			for req in &self.ctx.window_requests {
				match *req {
					WindowRequest::Fullscreen(b) => {
						if b {
							window.set_fullscreen(Some(window.get_current_monitor()));
						} else {
							window.set_fullscreen(None);
						}
						self.ctx.fullscreen = b;
					},
					WindowRequest::HideCursor(b) => {
						window.hide_cursor(b);
						self.ctx.cursor_hidden = b;
					},
					WindowRequest::LockCursor(b) => {
						window.grab_cursor(b)?;
						self.ctx.cursor_locked = b;
					},
					WindowRequest::SetTitle(ref s) => {
						window.set_title(&s);
						self.ctx.title = s.to_owned();
					},
				}
			}

			self.ctx.window_requests.clear();

			let actual_dt = start_time.elapsed();
			let actual_dt = actual_dt.as_millis() as f32;
			let expected_dt = 1000.0 / self.ctx.fps_cap as f32;

			if expected_dt > actual_dt {
				self.ctx.dt = expected_dt as f32 / 1000.0;
				thread::sleep(Duration::from_millis((expected_dt - actual_dt) as u64));
			} else {
				self.ctx.dt = actual_dt as f32 / 1000.0;
			}

			self.ctx.time += self.ctx.dt;

			if self.ctx.closed {
				break;
			}

		}

		return Ok(());

	}

}

pub fn str_to_key(s: &str) -> Option<Key> {

	return match s {
		"q" => Some(Key::Q),
		"w" => Some(Key::W),
		"e" => Some(Key::E),
		"r" => Some(Key::R),
		"t" => Some(Key::T),
		"y" => Some(Key::Y),
		"u" => Some(Key::U),
		"i" => Some(Key::I),
		"o" => Some(Key::O),
		"p" => Some(Key::P),
		"a" => Some(Key::A),
		"s" => Some(Key::S),
		"d" => Some(Key::D),
		"f" => Some(Key::F),
		"g" => Some(Key::G),
		"h" => Some(Key::H),
		"j" => Some(Key::J),
		"k" => Some(Key::K),
		"l" => Some(Key::L),
		"z" => Some(Key::Z),
		"x" => Some(Key::X),
		"c" => Some(Key::C),
		"v" => Some(Key::V),
		"b" => Some(Key::B),
		"n" => Some(Key::N),
		"m" => Some(Key::M),
		"1" => Some(Key::Key1),
		"2" => Some(Key::Key2),
		"3" => Some(Key::Key3),
		"4" => Some(Key::Key4),
		"5" => Some(Key::Key5),
		"6" => Some(Key::Key6),
		"7" => Some(Key::Key7),
		"8" => Some(Key::Key8),
		"9" => Some(Key::Key9),
		"0" => Some(Key::Key0),
		"f1" => Some(Key::F1),
		"f2" => Some(Key::F2),
		"f3" => Some(Key::F3),
		"f4" => Some(Key::F4),
		"f5" => Some(Key::F5),
		"f6" => Some(Key::F6),
		"f7" => Some(Key::F7),
		"f8" => Some(Key::F8),
		"f9" => Some(Key::F9),
		"f10" => Some(Key::F10),
		"f11" => Some(Key::F11),
		"f12" => Some(Key::F12),
		"-" => Some(Key::Minus),
		"=" => Some(Key::Equals),
		"," => Some(Key::Comma),
		"." => Some(Key::Period),
		"`" => Some(Key::Grave),
		"/" => Some(Key::Slash),
		"\\" => Some(Key::Backslash),
		";" => Some(Key::Semicolon),
		"'" => Some(Key::Apostrophe),
		"up" => Some(Key::Up),
		"down" => Some(Key::Down),
		"left" => Some(Key::Left),
		"right" => Some(Key::Right),
		"esc" => Some(Key::Escape),
		"tab" => Some(Key::Tab),
		"space" => Some(Key::Space),
		"back" => Some(Key::Back),
		"return" => Some(Key::Return),
		"lshift" => Some(Key::LShift),
		"rshift" => Some(Key::RShift),
		"lalt" => Some(Key::LAlt),
		"ralt" => Some(Key::RAlt),
		"lwin" => Some(Key::LWin),
		"rwin" => Some(Key::RWin),
		"lctrl" => Some(Key::LControl),
		"rctrl" => Some(Key::RControl),
		_ => None,
	};

}

pub fn str_to_mouse(s: &str) -> Option<Mouse> {
	return match s {
		"left" => Some(Mouse::Left),
		"right" => Some(Mouse::Right),
		"middle" => Some(Mouse::Middle),
		_ => None,
	}
}

