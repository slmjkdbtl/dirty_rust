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

pub struct Window {
	key_states: HashMap<Key, ButtonState>,
	rpressed_key: Option<Key>,
	text_input: Option<String>,
	mouse_pos: MousePos,
	mouse_delta: Option<MouseDelta>,
	scroll_delta: Option<ScrollDelta>,
	mouse_states: HashMap<Mouse, ButtonState>,
	resized: Option<Size>,
	event_loop: glutin::EventsLoop,
	windowed_ctx: glutin::WindowedContext,
	fullscreen: bool,
	relative: bool,
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

pub struct Conf {
	width: u32,
	height: u32,
	title: String,
	hidpi: bool,
	resizable: bool,
	fullscreen: bool,
	fullsize_content: bool,
}

impl Window {

	pub fn new(title: &str, width: u32, height: u32) -> Self {

		let event_loop = glutin::EventsLoop::new();

		let mut wbuilder = glutin::WindowBuilder::new()
			.with_title(title)
			.with_dimensions(LogicalSize::new(width as f64, height as f64));

		#[cfg(target_os = "macos")] {

			use glutin::os::macos::WindowBuilderExt;

			wbuilder = wbuilder
				.with_titlebar_transparent(true);
// 				.with_fullsize_content_view(true)
// 				.with_disallow_hidpi(true);

		}

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
			rpressed_key: None,
			text_input: None,
			key_states: HashMap::new(),
			mouse_states: HashMap::new(),
			mouse_pos: MousePos::new(0, 0),
			mouse_delta: None,
			scroll_delta: None,
			resized: None,
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

	pub fn set_pos(&self, pos: Vec2) {
		self.windowed_ctx.set_position(pos.into());
	}

	pub fn get_pos(&self) -> Vec2 {
		return self.windowed_ctx
			.get_position()
			.expect("cannot get window position")
			.into();
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

	pub fn key_rpressed(&self, key: Key) -> bool {
		return self.rpressed_key == Some(key);
	}

	pub fn rpressed_key(&self) -> Option<Key> {
		return self.rpressed_key;
	}

	pub fn text_input(&self) -> Option<String> {
		return self.text_input.clone();
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

	/// get if window was resized last frame
	pub fn resized(&self) -> Option<Size> {
		return self.resized;
	}

	pub fn size(&self) -> Size {
		return self.windowed_ctx
			.get_inner_size()
			.expect("failed to get window size")
			.into();
	}

	pub fn resize(&self, w: u32, h: u32) {
		self.windowed_ctx.set_inner_size(LogicalSize::new(w as f64, h as f64));
	}

	pub fn poll(&mut self) -> bool {

		let mut quit = false;
		let mut key_input = None;
		let mut mouse_input = None;
		let mut mouse_pos = None;
		let mut scroll_delta = None;
		let mut text_input = None;
		let mut device_mouse_delta = None;
		let mut resized = None;

		self.event_loop.poll_events(|event| {

			match event {

				glutin::Event::WindowEvent { event, .. } => match event {

					WindowEvent::CloseRequested => {
						quit = true;
					},

					WindowEvent::ReceivedCharacter(ch) => {
						text_input.get_or_insert(String::new()).push(ch);
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

					WindowEvent::Resized(size) => {
						resized = Some(size);
					},

					WindowEvent::Touch(touch) => {
						// ...
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

		self.rpressed_key = None;
		self.mouse_delta = None;
		self.scroll_delta = None;
		self.resized = None;
		self.text_input = text_input;

		if let Some(size) = resized {
			self.resized = Some(size.into());
		}

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
					self.rpressed_key = Some(key_code);
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

		#[cfg(target_os = "macos")] {
			if key_down(Key::RWin) || key_down(Key::LWin) {
				if key_pressed(Key::Q) {
					return false;
				}
			}
		}

		return true;

	}

	pub fn swap(&self) {
		self.windowed_ctx.swap_buffers();
	}

}

ctx!(WINDOW: Window);

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
	if gfx::enabled() {
		gfx::begin();
	}
}

pub fn end() {
	if gfx::enabled() {
		gfx::end();
	}
	ctx_get!(WINDOW).swap();
}

expose!(WINDOW, size() -> Size);
expose!(WINDOW, swap());
expose!(WINDOW, down_keys() -> HashSet<Key>);
expose!(WINDOW, rpressed_key() -> Option<Key>);
expose!(WINDOW, key_down(key: Key) -> bool);
expose!(WINDOW, key_pressed(key: Key) -> bool);
expose!(WINDOW, key_released(key: Key) -> bool);
expose!(WINDOW, key_rpressed(key: Key) -> bool);
expose!(WINDOW, text_input() -> Option<String>);
expose!(WINDOW, mouse_down(mouse: Mouse) -> bool);
expose!(WINDOW, mouse_pressed(mouse: Mouse) -> bool);
expose!(WINDOW, mouse_released(mouse: Mouse) -> bool);
expose!(WINDOW, mouse_pos() -> MousePos);
expose!(WINDOW, set_mouse_pos(pos: MousePos));
expose!(WINDOW, mouse_delta() -> Option<MouseDelta>);
expose!(WINDOW, scroll_delta() -> Option<ScrollDelta>);
expose!(WINDOW, resized() -> Option<Size>);
expose!(WINDOW(mut), set_fullscreen(b: bool));
expose!(WINDOW, is_fullscreen() -> bool);
expose!(WINDOW(mut), set_relative(b: bool));
expose!(WINDOW, is_relative() -> bool);
expose!(WINDOW, resize(w: u32, h: u32));
expose!(WINDOW, set_pos(pos: Vec2));
expose!(WINDOW, get_pos() -> Vec2);

