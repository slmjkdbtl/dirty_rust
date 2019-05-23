// wengwengweng

use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use std::time::Duration;

use glutin::dpi::*;
use glutin::Api;
use glutin::GlRequest;
use glutin::MouseScrollDelta;
pub use glutin::ModifiersState as Mod;
pub use glutin::VirtualKeyCode as Key;
pub use glutin::MouseButton as Mouse;
use derive_more::*;
use serde::Serialize;
use serde::Deserialize;

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
	fullscreen: bool,
	relative: bool,
	fps_cap: u32,
	conf: Conf,
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

#[derive(Serialize, Deserialize, Clone)]
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
		};
	}

}

impl Default for Window {
	fn default() -> Self {
		return Self::new(Conf::default());
	}
}

#[derive(Clone)]
pub struct Ctx {
	dt: f32,
	time: f32,
	closed: bool,
}

impl Ctx {

	fn new() -> Self {
		return Self {
			dt: 0.0,
			time: 0.0,
			closed: false,
		};
	}

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

}

impl Window {

	pub fn new(conf: Conf) -> Self {

		return Self {
			conf: conf,
			rpressed_key: None,
			text_input: None,
			key_states: HashMap::new(),
			mouse_states: HashMap::new(),
			mouse_pos: MousePos::new(0, 0),
			mouse_delta: None,
			scroll_delta: None,
			resized: None,
			fullscreen: false,
			relative: false,
			fps_cap: 60,
		};

	}

	pub fn run(&mut self, mut f: impl FnMut(&mut Ctx)) -> Result<(), Error> {

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
			let windowed_ctx = windowed_ctx.make_current().expect("cannot create context");
			gl::load_with(|symbol| windowed_ctx.get_proc_address(symbol) as *const _);
			windowed_ctx
		};

		let mut ctx = Ctx::new();

		loop {

			let mut quit = false;
			let start_time = Instant::now();

			use glutin::Event::*;
			use glutin::WindowEvent::*;
			use glutin::ControlFlow;

			event_loop.poll_events(|e| {

				match e {

					WindowEvent { event, .. } => {
						match event {
							CloseRequested => ctx.close(),
							_ => {},
						}
					},
					_ => {},

				};

			});

			f(&mut ctx);

			let actual_dt = start_time.elapsed();
			let actual_dt = actual_dt.as_millis() as f32;
			let expected_dt = 1000.0 / self.fps_cap as f32;

			if expected_dt > actual_dt {
				ctx.dt = expected_dt as f32 / 1000.0;
				thread::sleep(Duration::from_millis((expected_dt - actual_dt) as u64));
			} else {
				ctx.dt = actual_dt as f32 / 1000.0;
			}

			ctx.time += ctx.dt;

			if ctx.closed {
				break;
			}

		}

		return Ok(());

	}

// 	pub fn run(&mut self, mut f: impl FnMut()) {

// 		self.running = true;

// 		loop {

// 			let start_time = Instant::now();

// 			if !self.poll() {
// 				break;
// 			}

// 			f();
// 			self.swap();

// 			let actual_dt = start_time.elapsed();
// 			let actual_dt = actual_dt.as_millis() as f32;
// 			let expected_dt = 1000.0 / self.fps_cap as f32;

// 			if expected_dt > actual_dt {
// 				self.dt = expected_dt as f32 / 1000.0;
// 				thread::sleep(Duration::from_millis((expected_dt - actual_dt) as u64));
// 			} else {
// 				self.dt = actual_dt as f32 / 1000.0;
// 			}

// 			self.time += self.dt;

// 			if !self.running {
// 				break;
// 			}

// 		}

// 	}

// 	pub fn set_fullscreen(&mut self, b: bool) {

// 		let ctx = &self.windowed_ctx;
// 		let window = ctx.window();

// 		if b {
// 			window.set_fullscreen(Some(window.get_current_monitor()));
// 		} else {
// 			window.set_fullscreen(None);
// 		}

// 		self.fullscreen = b;

// 	}

	pub fn is_fullscreen(&self) -> bool {
		return self.fullscreen;
	}

// 	pub fn toggle_fullscreen(&mut self) {

// 		if self.is_fullscreen() {
// 			self.set_fullscreen(false);
// 		} else {
// 			self.set_fullscreen(true);
// 		}

// 	}

// 	pub fn set_relative(&mut self, b: bool) {

// 		let window = self.windowed_ctx.window();

// 		self.relative = b;
// 		window.hide_cursor(b);
// 		window.grab_cursor(b);

// 	}

	pub fn is_relative(&self) -> bool {
		return self.relative;
	}

// 	pub fn toggle_relative(&mut self) {

// 		if self.is_relative() {
// 			self.set_relative(false);
// 		} else {
// 			self.set_relative(true);
// 		}

// 	}

// 	pub fn set_mouse_pos(&self, pos: MousePos) {
// 		self.windowed_ctx.window().set_cursor_position(pos.into());
// 	}

// 	pub fn hide(&self) {
// 		self.windowed_ctx.window().hide();
// 	}

// 	pub fn show(&self) {
// 		self.windowed_ctx.window().show();
// 	}

// 	pub fn dpi(&self) -> f64 {
// 		return self.windowed_ctx.window().get_hidpi_factor();
// 	}

// 	pub fn set_pos(&self, pos: Vec2) {
// 		self.windowed_ctx.window().set_position(pos.into());
// 	}

// 	pub fn get_pos(&self) -> Vec2 {
// 		return self.windowed_ctx
// 			.window()
// 			.get_position()
// 			.expect("cannot get window position")
// 			.into();
// 	}

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

// 	pub fn size(&self) -> Size {
// 		return self.windowed_ctx
// 			.window()
// 			.get_inner_size()
// 			.expect("failed to get window size")
// 			.into();
// 	}

// 	pub fn resize(&self, w: u32, h: u32) {
// 		self.windowed_ctx.window().set_inner_size(LogicalSize::new(w as f64, h as f64));
// 	}

// 	pub fn poll(&mut self) -> bool {

// 		let mut quit = false;
// 		let mut key_input = None;
// 		let mut mouse_input = None;
// 		let mut mouse_pos = None;
// 		let mut scroll_delta = None;
// 		let mut text_input = None;
// 		let mut device_mouse_delta = None;
// 		let mut resized = None;

// 		self.event_loop.poll_events(|event| {

// 			match event {

// 				glutin::Event::WindowEvent { event, .. } => match event {

// 					WindowEvent::CloseRequested => {
// 						quit = true;
// 					},

// 					WindowEvent::ReceivedCharacter(ch) => {
// 						text_input.get_or_insert(String::new()).push(ch);
// 					},

// 					WindowEvent::CursorMoved { position, .. } => {
// 						mouse_pos = Some(position);
// 					},

// 					WindowEvent::MouseWheel { delta, .. } => {
// 						scroll_delta = Some(delta);
// 					},

// 					WindowEvent::MouseInput { button, state, .. } => {
// 						mouse_input = Some((button, state));
// 					},

// 					WindowEvent::KeyboardInput { input, .. } => {
// 						key_input = Some(input);
// 					},

// 					WindowEvent::Resized(size) => {
// 						resized = Some(size);
// 					},

// 					WindowEvent::Touch(touch) => {
// 					},

// 					_ => (),

// 				},

// 				glutin::Event::DeviceEvent { event, .. } => match event {
// 					DeviceEvent::MouseMotion { delta } => {
// 						device_mouse_delta = Some(delta);
// 					},
// 					_ => (),
// 				},

// 				_ => (),

// 			}

// 		});

// 		if quit {
// 			return false;
// 		}

// 		for (_, state) in &mut self.key_states {
// 			match state {
// 				ButtonState::Pressed => {
// 					*state = ButtonState::Down;
// 				},
// 				ButtonState::Released => {
// 					*state = ButtonState::Up;
// 				},
// 				_ => {}
// 			}
// 		}

// 		for (_, state) in &mut self.mouse_states {
// 			match state {
// 				ButtonState::Pressed => {
// 					*state = ButtonState::Down;
// 				},
// 				ButtonState::Released => {
// 					*state = ButtonState::Up;
// 				},
// 				_ => {}
// 			}
// 		}

// 		self.rpressed_key = None;
// 		self.mouse_delta = None;
// 		self.scroll_delta = None;
// 		self.resized = None;
// 		self.text_input = text_input;

// 		if let Some(size) = resized {
// 			self.resized = Some(size.into());
// 		}

// 		if let Some(scroll_delta) = scroll_delta {
// 			self.scroll_delta = Some(scroll_delta.into());
// 		}

// 		if let Some(mouse_pos) = mouse_pos {

// 			let prev_pos = self.mouse_pos;

// 			self.mouse_pos = mouse_pos.into();
// 			let delta = MouseDelta::new(self.mouse_pos.x - prev_pos.x, self.mouse_pos.y - prev_pos.y);

// 			if delta.x != 0 && delta.y != 0 {
// 				self.mouse_delta = Some(delta);
// 			}

// 		}

// 		if let Some(device_mouse_delta) = device_mouse_delta {
// 			self.mouse_delta = Some(MouseDelta {
// 				x: device_mouse_delta.0 as i32,
// 				y: device_mouse_delta.1 as i32,
// 			});
// 		}

// 		if let Some(key_input) = key_input {

// 			if let Some(key_code) = key_input.virtual_keycode {

// 				if key_input.state == ElementState::Pressed {
// 					self.rpressed_key = Some(key_code);
// 				}

// 				if let Some(state) = self.key_states.get_mut(&key_code) {

// 					match key_input.state {
// 						ElementState::Released => {
// 							if state == &ButtonState::Down {
// 								*state = ButtonState::Released;
// 							}
// 						},
// 						ElementState::Pressed => {
// 							if state == &ButtonState::Up {
// 								*state = ButtonState::Pressed;
// 							}
// 						}
// 					}

// 				} else {

// 					if key_input.state == ElementState::Pressed {
// 						self.key_states.insert(key_code, ButtonState::Pressed);
// 					}

// 				}

// 			}

// 		}

// 		if let Some((button, estate)) = mouse_input {

// 			if let Some(state) = self.mouse_states.get_mut(&button) {

// 				match estate {
// 					ElementState::Released => {
// 						if state == &ButtonState::Down {
// 							*state = ButtonState::Released;
// 						}
// 					},
// 					ElementState::Pressed => {
// 						if state == &ButtonState::Up {
// 							*state = ButtonState::Pressed;
// 						}
// 					}
// 				}

// 			} else {

// 				if estate == ElementState::Pressed {
// 					self.mouse_states.insert(button, ButtonState::Pressed);
// 				}

// 			}

// 		}

// 		return true;

// 	}

	/// set fps cap
	pub fn cap_fps(&mut self, cap: u32) {
		self.fps_cap = cap;
	}

// 	fn swap(&self) {
// 		self.windowed_ctx.swap_buffers();
// 	}

}

