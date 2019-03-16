// wengwengweng

use std::collections::HashMap;
use std::collections::HashSet;

use gctx::*;
use glutin::dpi::*;
use glutin::WindowEvent;
use glutin::DeviceEvent;
use glutin::KeyboardInput;
use glutin::Api;
use glutin::GlRequest;
use glutin::ElementState;
use glutin::ContextTrait;
use glutin::MouseScrollDelta;
pub use glutin::ModifiersState as Mod;
pub use glutin::VirtualKeyCode as Key;
pub use glutin::MouseButton as Mouse;
use derive_more::*;

use crate::math::*;
use crate::*;

ctx!(WINDOW: Window);

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
	fn new(x: i32, y: i32) -> Self {
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

#[derive(Copy, Clone, PartialEq, Debug, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, From, Into)]
pub struct ScrollDelta {
	pub x: i32,
	pub y: i32,
}

impl ScrollDelta {

	fn new(x: i32, y: i32) -> Self {
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

impl From<MouseScrollDelta> for ScrollDelta {
	fn from(delta: MouseScrollDelta) -> Self {
		return match delta {
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

pub struct Window {
	key_states: HashMap<Key, ButtonState>,
	key_input: Option<Key>,
	char_input: Option<char>,
	mouse_pos: MousePos,
	mouse_delta: Option<MouseDelta>,
	scroll_delta: Option<ScrollDelta>,
	mouse_states: HashMap<Mouse, ButtonState>,
	event_loop: glutin::EventsLoop,
	windowed_ctx: glutin::WindowedContext,
	fullscreen: bool,
	relative: bool,
}

impl Window {

	pub fn new(title: &str, width: u32, height: u32) -> Self {

		let event_loop = glutin::EventsLoop::new();

		let wbuilder = glutin::WindowBuilder::new()
			.with_title(title)
			.with_dimensions(LogicalSize::new(width as f64, height as f64));

		let windowed_ctx = glutin::ContextBuilder::new()
			.with_vsync(true)
			.with_gl(GlRequest::Specific(Api::OpenGl, (2, 1)))
			.build_windowed(wbuilder, &event_loop)
			.unwrap();

		unsafe {
			windowed_ctx.make_current().unwrap();
			gl::load_with(|symbol| windowed_ctx.get_proc_address(symbol) as *const _);
		}

		return Self {
			event_loop: event_loop,
			key_input: None,
			char_input: None,
			key_states: HashMap::new(),
			mouse_states: HashMap::new(),
			mouse_pos: MousePos::new(0, 0),
			mouse_delta: None,
			scroll_delta: None,
			windowed_ctx: windowed_ctx,
			fullscreen: false,
			relative: false,
		};

	}

	pub fn set_fullscreen(&mut self, b: bool) {

		let ctx = &self.windowed_ctx;

		if b {
			ctx.set_fullscreen(Some(ctx.get_current_monitor()));
		} else {
			ctx.set_fullscreen(None);
		}

		self.fullscreen = b;

	}

	pub fn is_fullscreen(&self) -> bool {
		return self.fullscreen;
	}

	pub fn set_relative(&mut self, b: bool) {

		self.relative = b;
		self.windowed_ctx.hide_cursor(b);
		self.windowed_ctx.grab_cursor(b);

	}

	pub fn is_relative(&self) -> bool {
		return self.relative;
	}

	pub fn set_mouse_pos(&self, pos: MousePos) {
		self.windowed_ctx.set_cursor_position(pos.into());
	}

	pub fn down_keys(&self) -> HashSet<Key> {
		return self.key_states
			.iter()
			.filter(|(_, &state)| state == ButtonState::Down)
			.map(|(key, _)| *key)
			.collect();
	}

	pub fn key_down(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Down);
	}

	pub fn key_pressed(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Pressed);
	}

	pub fn key_released(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Released);
	}

	pub fn key_pressed_repeat(&self, key: Key) -> bool {
		return self.key_input == Some(key);
	}

	pub fn char_input(&self) -> Option<char> {
		return self.char_input;
	}

	pub fn mouse_down(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Down);
	}

	pub fn mouse_pressed(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Pressed);
	}

	pub fn mouse_released(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Released);
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

	pub fn size(&self) -> Size {
		if let Some(size) = self.windowed_ctx.get_inner_size() {
			return size!(size.width, size.height);
		}
		return size!(0, 0);
	}

	pub fn poll(&mut self) -> bool {

		let mut quit = false;
		let mut key_input = None;
		let mut mouse_input = None;
		let mut char_input = None;
		let mut mouse_pos = None;
		let mut scroll_delta = None;
		let mut device_mouse_delta = None;

		self.event_loop.poll_events(&mut |event| {

			match event {

				glutin::Event::WindowEvent { event, .. } => match event {

					WindowEvent::CloseRequested => {
						quit = true;
					},

					WindowEvent::ReceivedCharacter(ch) => {
						char_input = Some(ch);
					},

					WindowEvent::CursorMoved { position, .. } => {
						mouse_pos = Some(position);
					},

					WindowEvent::MouseWheel { delta, .. } => {
						scroll_delta = Some(delta);
					},

					WindowEvent::MouseInput { button, state, .. } => {
						mouse_input = Some((button, state));
					},

					WindowEvent::KeyboardInput { input, .. } => {
						key_input = Some(input);
					},

					_ => (),

				},

				glutin::Event::DeviceEvent { event, .. } => match event {
					DeviceEvent::MouseMotion { delta } => {
						device_mouse_delta = Some(delta);
					},
					_ => (),
				},

				_ => (),

			}

		});

		if quit {
			return false;
		}

		for (_, state) in &mut self.key_states {
			match state {
				ButtonState::Pressed => {
					*state = ButtonState::Down;
				},
				ButtonState::Released => {
					*state = ButtonState::Up;
				},
				_ => {}
			}
		}

		for (_, state) in &mut self.mouse_states {
			match state {
				ButtonState::Pressed => {
					*state = ButtonState::Down;
				},
				ButtonState::Released => {
					*state = ButtonState::Up;
				},
				_ => {}
			}
		}

		self.key_input = None;
		self.mouse_delta = None;
		self.scroll_delta = None;
		self.char_input = char_input;

		if let Some(scroll_delta) = scroll_delta {
			self.scroll_delta = Some(scroll_delta.into());
		}

		if let Some(mouse_pos) = mouse_pos {

			let prev_pos = self.mouse_pos;

			self.mouse_pos = mouse_pos.into();
			let delta = MouseDelta::new(self.mouse_pos.x - prev_pos.x, self.mouse_pos.y - prev_pos.y);

			if delta.x != 0 && delta.y != 0 {
				self.mouse_delta = Some(delta);
			}

		}

		if let Some(device_mouse_delta) = device_mouse_delta {
			self.mouse_delta = Some(MouseDelta {
				x: device_mouse_delta.0 as i32,
				y: device_mouse_delta.1 as i32,
			});
		}

		if let Some(key_input) = key_input {

			if let Some(key_code) = key_input.virtual_keycode {

				if key_input.state == ElementState::Pressed {
					self.key_input = Some(key_code);
				}

				if let Some(state) = self.key_states.get_mut(&key_code) {

					match key_input.state {
						ElementState::Released => {
							if state == &ButtonState::Down {
								*state = ButtonState::Released;
							}
						},
						ElementState::Pressed => {
							if state == &ButtonState::Up {
								*state = ButtonState::Pressed;
							}
						}
					}

				} else {

					if key_input.state == ElementState::Pressed {
						self.key_states.insert(key_code, ButtonState::Pressed);
					}

				}

			}

		}

		if let Some((button, estate)) = mouse_input {

			if let Some(state) = self.mouse_states.get_mut(&button) {

				match estate {
					ElementState::Released => {
						if state == &ButtonState::Down {
							*state = ButtonState::Released;
						}
					},
					ElementState::Pressed => {
						if state == &ButtonState::Up {
							*state = ButtonState::Pressed;
						}
					}
				}

			} else {

				if estate == ElementState::Pressed {
					self.mouse_states.insert(button, ButtonState::Pressed);
				}

			}

		}

		return true;

	}

	pub fn swap(&self) {
		self.windowed_ctx.swap_buffers();
	}

}

pub fn init(title: &str, width: u32, height: u32) {
	ctx_init!(WINDOW, Window::new(title, width, height));
	gfx::init();
}

pub fn enabled() -> bool {
	return ctx_ok!(WINDOW);
}

pub fn begin() {
	if !ctx_mut!(WINDOW).poll() {
		app::quit();
	}
	gfx::begin();
}

pub fn end() {
	gfx::end();
	ctx_get!(WINDOW).swap();
}

pub fn size() -> Size {
	return ctx_get!(WINDOW).size();
}

pub fn swap() {
	return ctx_get!(WINDOW).swap();
}

pub fn down_keys() -> HashSet<Key> {
	return ctx_get!(WINDOW).down_keys();
}

pub fn key_down(key: Key) -> bool {
	return ctx_get!(WINDOW).key_down(key);
}

pub fn key_pressed(key: Key) -> bool {
	return ctx_get!(WINDOW).key_pressed(key);
}

pub fn key_released(key: Key) -> bool {
	return ctx_get!(WINDOW).key_released(key);
}

pub fn key_pressed_repeat(key: Key) -> bool {
	return ctx_get!(WINDOW).key_pressed_repeat(key);
}

pub fn char_input() -> Option<char> {
	return ctx_get!(WINDOW).char_input();
}

pub fn mouse_down(mouse: Mouse) -> bool {
	return ctx_get!(WINDOW).mouse_down(mouse);
}

pub fn mouse_pressed(mouse: Mouse) -> bool {
	return ctx_get!(WINDOW).mouse_pressed(mouse);
}

pub fn mouse_released(mouse: Mouse) -> bool {
	return ctx_get!(WINDOW).mouse_released(mouse);
}

pub fn set_fullscreen(b: bool) {
	return ctx_mut!(WINDOW).set_fullscreen(b);
}

pub fn is_fullscreen() -> bool {
	return ctx_get!(WINDOW).is_fullscreen();
}

pub fn set_relative(b: bool) {
	return ctx_mut!(WINDOW).set_relative(b);
}

pub fn is_relative() -> bool {
	return ctx_mut!(WINDOW).is_relative();
}

pub fn mouse_pos() -> MousePos {
	return ctx_get!(WINDOW).mouse_pos();
}

pub fn mouse_delta() -> Option<MouseDelta> {
	return ctx_get!(WINDOW).mouse_delta();
}

pub fn scroll_delta() -> Option<ScrollDelta> {
	return ctx_get!(WINDOW).scroll_delta();
}

