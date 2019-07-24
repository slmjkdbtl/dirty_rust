// wengwengweng

//! Window & Graphics

use std::collections::HashMap;
use std::collections::HashSet;

use glutin::dpi::*;
use glutin::Api;
use glutin::GlRequest;
use glutin::ElementState;
use derive_more::*;
use gilrs::Gilrs;

pub use glutin::ModifiersState as Mod;
pub use glutin::VirtualKeyCode as Key;
pub use glutin::MouseButton as Mouse;

use crate::math::*;
use crate::*;

/// Manages Ctx
pub struct Ctx {

	key_state: HashMap<Key, ButtonState>,
	mouse_state: HashMap<Mouse, ButtonState>,
	mouse_pos: Pos,
	mouse_delta: Option<Pos>,
	scroll_delta: Option<Pos>,
	text_input: Option<String>,
	title: String,
	fullscreen: bool,
	cursor_hidden: bool,
	cursor_locked: bool,
	should_quit: bool,

	pub(crate) windowed_ctx: glutin::WindowedContext<glutin::PossiblyCurrent>,
	events_loop: glutin::EventsLoop,
	gamepad_ctx: gilrs::Gilrs,

}

impl Ctx {

	pub fn new(conf: &app::Conf) -> Result<Self> {

		let mut events_loop = glutin::EventsLoop::new();

		let mut window_builder = glutin::WindowBuilder::new()
			.with_title(conf.title.to_owned())
			.with_resizable(conf.resizable)
			.with_transparency(conf.transparent)
			.with_decorations(!conf.borderless)
			.with_always_on_top(conf.always_on_top)
			.with_dimensions(LogicalSize::new(conf.width as f64, conf.height as f64))
			.with_multitouch();

		if conf.fullscreen {
			window_builder = window_builder
				.with_fullscreen(Some(events_loop.get_primary_monitor()));
		}

		#[cfg(target_os = "macos")] {

			use glutin::os::macos::WindowBuilderExt;

			window_builder = window_builder
				.with_titlebar_buttons_hidden(conf.hide_titlebar_buttons)
				.with_title_hidden(conf.hide_title)
				.with_titlebar_transparent(conf.titlebar_transparent)
				.with_fullsize_content_view(conf.fullsize_content);

		}

		let ctx_builder = glutin::ContextBuilder::new()
			.with_vsync(conf.vsync)
			.with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)));

		let windowed_ctx = ctx_builder
			.build_windowed(window_builder, &events_loop)?;

		let windowed_ctx = unsafe { windowed_ctx.make_current()? };
		let window = windowed_ctx.window();

		let mut gamepad_ctx = Gilrs::new()?;

		windowed_ctx.swap_buffers()?;

		return Ok(Self {

			key_state: HashMap::new(),
			mouse_state: HashMap::new(),
			mouse_pos: Pos::new(0, 0),
			mouse_delta: None,
			scroll_delta: None,
			text_input: None,
			fullscreen: conf.fullscreen,
			cursor_hidden: conf.cursor_hidden,
			cursor_locked: conf.cursor_locked,
			title: conf.title.to_owned(),
			should_quit: false,

			events_loop: events_loop,
			windowed_ctx: windowed_ctx,
			gamepad_ctx: gamepad_ctx,

		});

	}

	pub(crate) fn poll(&mut self) -> Result<()> {

		for state in self.key_state.values_mut() {
			if state == &ButtonState::Pressed {
				*state = ButtonState::Down;
			} else if state == &ButtonState::Released {
				*state = ButtonState::Up;
			}
		}

		for state in self.mouse_state.values_mut() {
			if state == &ButtonState::Pressed {
				*state = ButtonState::Down;
			} else if state == &ButtonState::Released {
				*state = ButtonState::Up;
			}
		}

		self.mouse_delta = None;
		self.scroll_delta = None;
		self.text_input = None;

		let mut keyboard_input = None;
		let mut mouse_input = None;
		let mut cursor_moved = None;
		let mut mouse_wheel = None;
		let mut text_input = None;
		let mut close = false;

		self.events_loop.poll_events(|e| {

			use glutin::Event::*;
			use glutin::WindowEvent::*;

			match e {

				WindowEvent { event, .. } => match event {

					KeyboardInput { input, .. } => {
						keyboard_input = Some(input);
					},

					MouseInput { button, state, .. } => {
						mouse_input = Some((button, state));
					},

					CursorMoved { position, .. } => {
						cursor_moved = Some(position);
					},

					MouseWheel { delta, .. } => {
						mouse_wheel = Some(delta);
					},

					ReceivedCharacter(ch) => {
						text_input.get_or_insert(String::new()).push(ch);
					},

					CloseRequested => close = true,

					_ => {},

				},

				_ => {},

			};

		});

		if close {
			self.should_quit = true
		}

		if let Some(input) = keyboard_input {
			if let Some(kc) = input.virtual_keycode {
				match input.state {
					ElementState::Pressed => {
						if self.key_up(kc) || self.key_released(kc) {
							self.key_state.insert(kc, ButtonState::Pressed);
						}
					},
					ElementState::Released => {
						if self.key_down(kc) || self.key_pressed(kc) {
							self.key_state.insert(kc, ButtonState::Released);
						}
					},
				}
			}
		}

		if let Some((button, state)) = mouse_input {
			match state {
				ElementState::Pressed => {
					if self.mouse_up(button) || self.mouse_released(button) {
						self.mouse_state.insert(button, ButtonState::Pressed);
					}
				},
				ElementState::Released => {
					if self.mouse_down(button) || self.mouse_pressed(button) {
						self.mouse_state.insert(button, ButtonState::Released);
					}
				},
			}
		}

		if let Some(pos) = cursor_moved {

			let pos: Pos = pos.into();

			self.mouse_delta = Some((pos - self.mouse_pos).into());
			self.mouse_pos = pos;

		}

		if let Some(delta) = mouse_wheel {
			self.scroll_delta = Some(delta.into());
		}

		self.text_input = text_input;

		while let Some(gilrs::Event { id, event, .. }) = self.gamepad_ctx.next_event() {
			// ...
		}

		return Ok(());

	}

	pub(crate) fn swap(&self) -> Result<()> {
		return Ok(self.windowed_ctx.swap_buffers()?);
	}

	pub(crate) fn should_quit(&self) -> bool {
		return self.should_quit;
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

	pub fn key_pressed_repeat(&self, key: Key) -> bool {
		unimplemented!();
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

	pub fn mouse_pos(&self) -> Pos {
		return self.mouse_pos;
	}

	pub fn mouse_delta(&self) -> Option<Pos> {
		return self.mouse_delta;
	}

	pub fn scroll_delta(&self) -> Option<Pos> {
		return self.scroll_delta;
	}

	pub fn text_input(&self) -> Option<String> {
		return self.text_input.clone();
	}

	pub fn set_fullscreen(&mut self, b: bool) {

		let window = self.windowed_ctx.window();

		if b {
			window.set_fullscreen(Some(window.get_current_monitor()));
			self.fullscreen = true;
		} else {
			window.set_fullscreen(None);
			self.fullscreen = false;
		}

	}

	pub fn is_fullscreen(&self) -> bool {
		return self.fullscreen;
	}

	pub fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	pub fn set_cursor_hidden(&mut self, b: bool) {
		self.windowed_ctx.window().hide_cursor(b);
		self.cursor_hidden = b;
	}

	pub fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	pub fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	pub fn set_cursor_locked(&mut self, b: bool) {
		self.windowed_ctx.window().grab_cursor(b);
		self.cursor_locked = b;
	}

	pub fn is_cursor_locked(&self) -> bool {
		return self.cursor_locked;
	}

	pub fn toggle_cursor_locked(&mut self) {
		self.set_cursor_locked(!self.is_cursor_locked());
	}

	pub fn set_title(&mut self, t: &str) {
		self.windowed_ctx.window().set_title(t);
	}

}

// expose!(window, size() -> Size);
expose!(window, down_keys() -> HashSet<Key>);
expose!(window, key_down(key: Key) -> bool);
expose!(window, key_pressed(key: Key) -> bool);
expose!(window, key_released(key: Key) -> bool);
expose!(window, key_pressed_repeat(key: Key) -> bool);
expose!(window, text_input() -> Option<String>);
expose!(window, mouse_down(mouse: Mouse) -> bool);
expose!(window, mouse_pressed(mouse: Mouse) -> bool);
expose!(window, mouse_released(mouse: Mouse) -> bool);
expose!(window, mouse_pos() -> Pos);
expose!(window, mouse_delta() -> Option<Pos>);
expose!(window, scroll_delta() -> Option<Pos>);
expose!(window(mut), set_fullscreen(b: bool));
expose!(window(mut), toggle_fullscreen());
expose!(window, is_fullscreen() -> bool);
expose!(window(mut), set_title(t: &str));

#[derive(Clone, Copy, Debug, PartialEq)]
enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

#[derive(Copy, Clone, PartialEq, Debug, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, From, Into)]
pub struct Pos {
	pub x: i32,
	pub y: i32,
}

impl Pos {
	fn new(x: i32, y: i32) -> Self {
		return Self {
			x: x,
			y: y,
		};
	}
}

impl From<Pos> for Vec2 {
	fn from(mpos: Pos) -> Self {
		return vec2!(mpos.x, mpos.y);
	}
}

impl From<LogicalPosition> for Pos {
	fn from(pos: LogicalPosition) -> Self {
		let (x, y): (i32, i32) = pos.into();
		return Self {
			x: x,
			y: y,
		};
	}
}

impl From<Pos> for LogicalPosition {
	fn from(pos: Pos) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

impl From<glutin::MouseScrollDelta> for Pos {
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

pub(crate) fn str_to_key(s: &str) -> Option<Key> {

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

pub(crate) fn str_to_mouse(s: &str) -> Option<Mouse> {
	return match s {
		"left" => Some(Mouse::Left),
		"right" => Some(Mouse::Right),
		"middle" => Some(Mouse::Middle),
		_ => None,
	}
}

