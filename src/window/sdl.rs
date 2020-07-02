// wengwengweng

use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::*;
use math::*;
use input::*;
use window::*;

/// The Window Context
pub struct Window {
	sdl_ctx: sdl2::Sdl,
	gl_ctx: sdl2::video::GLContext,
	window: sdl2::video::Window,
	video_sys: sdl2::VideoSubsystem,
	gl: Rc<glow::Context>,
	pressed_keys: HashSet<Key>,
	pressed_mouse: HashSet<Mouse>,
	gamepad_pressed_buttons: HashMap<GamepadID, HashSet<GamepadButton>>,
	gamepad_axis_pos: HashMap<GamepadID, HashMap<GamepadAxis, Vec2>>,
	width: i32,
	height: i32,
	mouse_pos: Vec2,
	focused: bool,
	quit: bool,
}

impl Window {

	pub(crate) fn new(conf: &conf::Conf) -> Result<Self> {

		let sdl_ctx = sdl2::init()?;
		let video_sys = sdl_ctx.video()?;
		let gl_attr = video_sys.gl_attr();

		gl_attr.set_context_profile(sdl2::video::GLProfile::Compatibility);
		gl_attr.set_context_version(2, 1);

		let mut window = video_sys
			.window(&conf.title, conf.width as u32, conf.height as u32);

		window.opengl();

		if conf.resizable {
			window.resizable();
		}

		if conf.fullscreen {
			window.fullscreen_desktop();
		}

		let window = window
			.build()
			.map_err(|e| e.to_string())?;

		let gl_ctx = window.gl_create_context()?;

		let gl = glow::Context::from_loader_function(|s| {
			return video_sys.gl_get_proc_address(s) as *const _;
		});

		let swap_interval = if conf.vsync {
			sdl2::video::SwapInterval::VSync
		} else {
			sdl2::video::SwapInterval::Immediate
		};

		video_sys.gl_set_swap_interval(swap_interval)?;

		if conf.cursor_hidden {
// 			sdl_ctx.mouse().show_cursor(false);
		}

		if conf.cursor_locked {
			sdl_ctx.mouse().set_relative_mouse_mode(true);
		}

		return Ok(Self {
			sdl_ctx: sdl_ctx,
			window: window,
			video_sys: video_sys,
			gl_ctx: gl_ctx,
			gl: Rc::new(gl),
			pressed_keys: hset![],
			pressed_mouse: hset![],
			gamepad_pressed_buttons: hmap![],
			gamepad_axis_pos: hmap![],
			mouse_pos: vec2!(),
			width: conf.width,
			height: conf.height,
			focused: true,
			quit: false,
		});

	}

}

impl Window {

	pub(crate) fn gl(&self) -> &Rc<glow::Context> {
		return &self.gl;
	}

	pub(crate) fn swap(&self) -> Result<()> {
		self.window.gl_swap_window();
		return Ok(());
	}

	pub fn focused(&self) -> bool {
		return self.focused;
	}

	/// check if a key is currently pressed
	pub fn key_down(&self, k: Key) -> bool {
		return self.pressed_keys.contains(&k);
	}

	/// get current ([KeyMod](input::KeyMod))
	pub fn key_mods(&self) -> KeyMod {
		return KeyMod {
			shift: self.key_down(Key::LShift) || self.key_down(Key::RShift),
			ctrl: self.key_down(Key::LCtrl) || self.key_down(Key::RCtrl),
			alt: self.key_down(Key::LAlt) || self.key_down(Key::RAlt),
			meta: self.key_down(Key::LMeta) || self.key_down(Key::RMeta),
		};
	}

	/// check if a mouse button is currently pressed
	pub fn mouse_down(&self, m: Mouse) -> bool {
		return self.pressed_mouse.contains(&m);
	}

	/// check if a gamepad button is currently pressed
	pub fn gamepad_down(&self, id: GamepadID, b: GamepadButton) -> bool {
		return self.gamepad_pressed_buttons
			.get(&id)
			.map(|bts| bts.contains(&b))
			.unwrap_or(false);
	}

	/// get gamepad axis position
	pub fn gamepad_axis(&self, id: GamepadID, axis: GamepadAxis) -> Vec2 {
		return self.gamepad_axis_pos
			.get(&id)
			.map(|p| p.get(&axis))
			.flatten()
			.map(|p| *p)
			.unwrap_or(vec2!(0));
	}

	/// get current dpi
	pub fn dpi(&self) -> f32 {
		return 1.0;
	}

	/// get current window width
	pub fn width(&self) -> i32 {
		return self.width;
	}

	/// get current window height
	pub fn height(&self) -> i32 {
		return self.height;
	}

	/// get current mouse position
	pub fn mouse_pos(&self) -> Vec2 {
		return self.mouse_pos;
	}

	/// set mouse position
	pub fn set_mouse_pos(&mut self, p: Vec2) -> Result<()> {
		return Ok(());
	}

	/// set fullscreen
	pub fn set_fullscreen(&mut self, b: bool) {
		let ty = if b {
			sdl2::video::FullscreenType::Desktop
		} else {
			sdl2::video::FullscreenType::Off
		};
		self.window.set_fullscreen(ty).ok();
	}

	/// check if is fullscreen
	pub fn is_fullscreen(&self) -> bool {
		return self.window.fullscreen_state() != sdl2::video::FullscreenType::Off;
	}

	/// set cursor hidden
	pub fn set_cursor_hidden(&mut self, b: bool) {
// 		self.sdl_ctx.mouse().show_cursor(!b);
	}

	/// check if is cursor hidden
	pub fn is_cursor_hidden(&self) -> bool {
		return self.sdl_ctx.mouse().is_cursor_showing();
	}

	/// set cursor locked
	pub fn set_cursor_locked(&mut self, b: bool) {
		self.sdl_ctx.mouse().set_relative_mouse_mode(b);
	}

	/// check if is cursor locked
	pub fn is_cursor_locked(&self) -> bool {
		return self.sdl_ctx.mouse().relative_mouse_mode();
	}

	/// set window title
	pub fn set_title(&mut self, s: &str) {
		self.window.set_title(s).ok();
	}

	/// get window title
	pub fn title(&self) -> &str {
		return self.window.title();
	}

	/// set cursor icon
	pub fn set_cursor(&mut self, c: CursorIcon) {
	}

	/// quit
	pub fn quit(&mut self) {
		self.quit = true;
	}

	pub(crate) fn run(
		mut self,
		mut handle: impl FnMut(&mut Self, WindowEvent) -> Result<()> + 'static,
	) -> Result<()> {

		#[cfg(feature = "midi")]
		let midi_rx = midi::listen()?;

		let mut event_pump = self.sdl_ctx
			.event_pump()
			.map_err(|e| e.to_string())?;

		loop {

			let mut events = vec![];
			let mut quit = self.quit;

			for event in event_pump.poll_iter() {
				use sdl2::event::Event as SDLEvent;
				match event {
					SDLEvent::Quit { .. } => {
						if !quit {
							quit = true;
						}
					},
					SDLEvent::Window { win_event, .. } => {
						match win_event {
							sdl2::event::WindowEvent::Resized(w, h) => {
								self.width = w;
								self.height = h;
								handle(&mut self, WindowEvent::Resize(w, h))?;
								events.push(Event::Resize(w, h));
							},
							_ => {},
						}
					},
					SDLEvent::KeyDown { keycode, repeat, .. } => {
						if let Some(kc) = keycode {
							if let Some(key) = Key::from_sdl(kc) {
								events.push(Event::KeyPressRepeat(key));
								if !self.key_down(key) {
									events.push(Event::KeyPress(key));
								}
								self.pressed_keys.insert(key);
							}
						}
					},
					SDLEvent::KeyUp { keycode, repeat, .. } => {
						if let Some(kc) = keycode {
							if let Some(key) = Key::from_sdl(kc) {
								events.push(Event::KeyRelease(key));
								self.pressed_keys.remove(&key);
							}
						}
					},
					SDLEvent::MouseMotion { x, y, xrel, yrel, .. } => {
						events.push(Event::MouseMove(vec2!(xrel, -yrel)));
						self.mouse_pos = vec2!(x - self.width / 2, self.height / 2 - y);
					},
					SDLEvent::MouseButtonDown { mouse_btn, .. } => {
						if let Some(m) = Mouse::from_sdl(mouse_btn) {
							self.pressed_mouse.insert(m);
							events.push(Event::MousePress(m));
						}
					},
					SDLEvent::MouseButtonUp { mouse_btn, .. } => {
						if let Some(m) = Mouse::from_sdl(mouse_btn) {
							self.pressed_mouse.remove(&m);
							events.push(Event::MouseRelease(m));
						}
					},
					SDLEvent::TextInput { text, .. } => {
						for ch in text.chars() {
							if !INVALID_CHARS.contains(&ch) && !ch.is_control() {
								events.push(Event::CharInput(ch));
							}
						}
					}
					_ => {}
				}
			}

			#[cfg(feature = "midi")]
			for msg in midi_rx.try_iter() {
				events.push(Event::MIDI(msg.clone()));
			}

			for e in events {
				handle(&mut self, WindowEvent::Input(e))?;
			}

			handle(&mut self, WindowEvent::Frame)?;
			self.swap()?;

			if quit {
				handle(&mut self, WindowEvent::Quit)?;
				break;
			}

		}

		return Ok(());

	}

	/// toggle fullscreen state
	pub fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	/// toggle cursor hidden state
	pub fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	/// toggle cursor lock state
	pub fn toggle_cursor_locked(&mut self) {
		self.set_cursor_locked(!self.is_cursor_locked());
	}

	/// minimize window
	pub fn minimize(&self) {
	}

}

impl Key {

	fn from_sdl(k: sdl2::keyboard::Keycode) -> Option<Self> {
		use sdl2::keyboard::Keycode::*;
		return match k {
			Q => Some(Key::Q),
			W => Some(Key::W),
			E => Some(Key::E),
			R => Some(Key::R),
			T => Some(Key::T),
			Y => Some(Key::Y),
			U => Some(Key::U),
			I => Some(Key::I),
			O => Some(Key::O),
			P => Some(Key::P),
			A => Some(Key::A),
			S => Some(Key::S),
			D => Some(Key::D),
			F => Some(Key::F),
			G => Some(Key::G),
			H => Some(Key::H),
			J => Some(Key::J),
			K => Some(Key::K),
			L => Some(Key::L),
			Z => Some(Key::Z),
			X => Some(Key::X),
			C => Some(Key::C),
			V => Some(Key::V),
			B => Some(Key::B),
			N => Some(Key::N),
			M => Some(Key::M),
			Num1 => Some(Key::Key1),
			Num2 => Some(Key::Key2),
			Num3 => Some(Key::Key3),
			Num4 => Some(Key::Key4),
			Num5 => Some(Key::Key5),
			Num6 => Some(Key::Key6),
			Num7 => Some(Key::Key7),
			Num8 => Some(Key::Key8),
			Num9 => Some(Key::Key9),
			Num0 => Some(Key::Key0),
			F1 => Some(Key::F1),
			F2 => Some(Key::F2),
			F3 => Some(Key::F3),
			F4 => Some(Key::F4),
			F5 => Some(Key::F5),
			F6 => Some(Key::F6),
			F7 => Some(Key::F7),
			F8 => Some(Key::F8),
			F9 => Some(Key::F9),
			F10 => Some(Key::F10),
			F11 => Some(Key::F11),
			F12 => Some(Key::F12),
			Minus => Some(Key::Minus),
			Equals => Some(Key::Equal),
			Comma => Some(Key::Comma),
			Period => Some(Key::Period),
			Quotedbl => Some(Key::Backquote),
			Slash => Some(Key::Slash),
			Backslash => Some(Key::Backslash),
			Semicolon => Some(Key::Semicolon),
			Quote => Some(Key::Quote),
			Up => Some(Key::Up),
			Down => Some(Key::Down),
			Left => Some(Key::Left),
			Right => Some(Key::Right),
			Escape => Some(Key::Esc),
			Tab => Some(Key::Tab),
			Space => Some(Key::Space),
			Backspace => Some(Key::Backspace),
			Return => Some(Key::Enter),
			LShift => Some(Key::LShift),
			RShift => Some(Key::RShift),
			LAlt => Some(Key::LAlt),
			RAlt => Some(Key::RAlt),
			LGui => Some(Key::LMeta),
			RGui => Some(Key::RMeta),
			LCtrl => Some(Key::LCtrl),
			RCtrl => Some(Key::RCtrl),
			_ => None,
		};
	}

}

impl Mouse {
	fn from_sdl(m: sdl2::mouse::MouseButton) -> Option<Self> {
		use sdl2::mouse::MouseButton::*;
		return match m {
			Left => Some(Mouse::Left),
			Right => Some(Mouse::Right),
			Middle => Some(Mouse::Middle),
			_ => None,
		};
	}
}

