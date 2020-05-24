// wengwengweng

use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;

use glutin::dpi::*;
use glutin::event_loop::EventLoop;

use crate::*;
use math::*;
use input::*;
use window::*;

/// The Window Context
pub struct Window {
	event_loop: Option<EventLoop<()>>,
	windowed_ctx: glutin::WindowedContext<glutin::PossiblyCurrent>,
	pressed_keys: HashSet<Key>,
	pressed_mouse: HashSet<Mouse>,
	pressed_gamepad_buttons: HashMap<GamepadID, HashSet<GamepadButton>>,
	gamepad_axis_pos: HashMap<GamepadID, (Vec2, Vec2)>,
	width: i32,
	height: i32,
	mouse_pos: Vec2,
	scroll_phase: input::ScrollPhase,
	cursor_hidden: bool,
	cursor_locked: bool,
	title: String,
	quit: bool,
	gamepad_ctx: gilrs::Gilrs,
}

impl Window {

	pub(crate) fn new(conf: &conf::Conf) -> Result<Self> {

		let event_loop = EventLoop::new();

		let mut window_builder = glutin::window::WindowBuilder::new()
			.with_title(conf.title.to_owned())
			.with_resizable(conf.resizable)
			.with_transparent(conf.transparent)
			.with_decorations(!conf.borderless)
			.with_always_on_top(conf.always_on_top)
			.with_inner_size(LogicalSize::new(conf.width as f64, conf.height as f64))
			;

		if conf.fullscreen {
			window_builder = window_builder
				.with_fullscreen(Some(glutin::window::Fullscreen::Borderless(event_loop.primary_monitor())));
		}

		#[cfg(target_os = "macos")] {

			use glutin::platform::macos::WindowBuilderExtMacOS;

			window_builder = window_builder
				.with_disallow_hidpi(!conf.hidpi)
				;

		}

		let ctx_builder = glutin::ContextBuilder::new()
			.with_vsync(conf.vsync)
			.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (2, 1)))
			;

		let windowed_ctx = unsafe {
			ctx_builder
				.build_windowed(window_builder, &event_loop)
				.map_err(|_| format!("failed to build window"))?
				.make_current()
				.map_err(|_| format!("failed to make opengl context"))?
		};

		if conf.cursor_hidden {
			windowed_ctx.window().set_cursor_visible(false);
		}

		if conf.cursor_locked {
			windowed_ctx.window().set_cursor_grab(true);
		}

		return Ok(Self {
			event_loop: Some(event_loop),
			windowed_ctx: windowed_ctx,
			pressed_keys: hset![],
			pressed_mouse: hset![],
			pressed_gamepad_buttons: hmap![],
			gamepad_axis_pos: hmap![],
			mouse_pos: vec2!(),
			width: conf.width,
			height: conf.height,
			scroll_phase: input::ScrollPhase::Solid,
			cursor_hidden: conf.cursor_hidden,
			cursor_locked: conf.cursor_locked,
			title: conf.title.to_string(),
			quit: false,
			gamepad_ctx: gilrs::Gilrs::new()
				.map_err(|_| format!("failed to create gamepad context"))?,
		});

	}

}

impl Window {

	pub(crate) fn get_gl_ctx(&self) -> Result<gl::Device> {

		return Ok(gl::Device::from_loader(|s| {
			return self.windowed_ctx.get_proc_address(s) as *const _;
		}));

	}

	pub(crate) fn swap(&self) -> Result<()> {
		self.windowed_ctx
			.swap_buffers()
			.map_err(|_| format!("failed to swap buffer"))?;
		return Ok(());
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

	/// get current dpi
	pub fn dpi(&self) -> f32 {
		return self.windowed_ctx.window().scale_factor() as f32;
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

		let (w, h) = (self.width as f32, self.height as f32);
		let mpos = vec2!(w / 2.0 + p.x, h / 2.0 - p.y);
		let g_mpos: LogicalPosition<f64> = mpos.into();

		self.windowed_ctx
			.window()
			.set_cursor_position(g_mpos)
			.map_err(|_| format!("failed to set mouse position"))?
			;

		self.mouse_pos = mpos;

		return Ok(());

	}

	/// set fullscreen
	pub fn set_fullscreen(&mut self, b: bool) {

		use glutin::window::Fullscreen;

		let window = self.windowed_ctx.window();

		if b {
			window.set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
		} else {
			window.set_fullscreen(None);
		}

	}

	/// check if is fullscreen
	pub fn is_fullscreen(&self) -> bool {
		return self.windowed_ctx.window().fullscreen().is_some();
	}

	/// set cursor hidden
	pub fn set_cursor_hidden(&mut self, b: bool) {
		self.windowed_ctx.window().set_cursor_visible(!b);
		self.cursor_hidden = b;
	}

	/// check if is cursor hidden
	pub fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	/// set cursor locked
	pub fn set_cursor_locked(&mut self, b: bool) {
		self.windowed_ctx.window().set_cursor_grab(b);
		self.cursor_locked = b;
	}

	/// check if is cursor locked
	pub fn is_cursor_locked(&self) -> bool {
		return self.cursor_locked;
	}

	/// set window title
	pub fn set_title(&mut self, s: &str) {
		self.title = s.to_owned();
		self.windowed_ctx.window().set_title(s);
	}

	/// get window title
	pub fn title(&self) -> &str {
		return &self.title;
	}

	/// set cursor icon
	pub fn set_cursor(&mut self, c: CursorIcon) {
		self.windowed_ctx.window().set_cursor_icon(c.to_winit());
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
		let midi_buf = {

			use std::sync::Mutex;
			use std::sync::Arc;

			let buf = Arc::new(Mutex::new(vec![]));
			let buf_in = buf.clone();

			// TODO: why does this still block the main thread sometime??
			thread::Builder::new()
				.name(String::from("dirty_midi"))
				.spawn(move || {

				// TODO: extremely slow
				if let Ok(midi_in) = midir::MidiInput::new("DIRTY") {

					if let Some(port) = midi_in.ports().last() {

						let port_name = midi_in.port_name(port).unwrap_or(format!("unknown"));

						let _conn = midi_in.connect(port, &format!("DIRTY ({})", port_name), move |_, msg, buf| {
							if let Ok(mut buf) = buf.lock() {
								buf.push(midi::Msg::from(&msg));
							}
						}, buf_in).map_err(|_| format!("failed to read midi input"));

						loop {}

					}

				} else {
					eprintln!("failed to init midi input")
				}

			});

			buf

		};

		use glutin::event_loop::ControlFlow;

		let mut update = false;

		let event_loop = match self.event_loop.take() {
			Some(e) => e,
			None => return Ok(()),
		};

		event_loop.run(move |e, _, flow| {

			if self.quit {
				*flow = ControlFlow::Exit;
				return;
			}

			*flow = ControlFlow::Poll;

			let res: Result<()> = try {

				use glutin::event::WindowEvent as WEvent;
				use glutin::event::DeviceEvent as DEvent;
				use glutin::event::TouchPhase;
				use glutin::event::ElementState;
				use input::*;

				let mut events = vec![];

				#[cfg(feature = "midi")]
				if let Ok(mut buf) = midi_buf.lock() {
					for msg in std::mem::replace(&mut *buf, vec![]) {
						events.push(Event::MIDI(msg.clone()));
					}
				}

				match e {

					glutin::event::Event::LoopDestroyed => *flow = ControlFlow::Exit,

					glutin::event::Event::WindowEvent { ref event, .. } => match event {

						WEvent::CloseRequested => {
							*flow = ControlFlow::Exit;
						},

						WEvent::ScaleFactorChanged { scale_factor, .. } => {
							handle(&mut self, WindowEvent::DPIChange(*scale_factor as f32))?;
						},

						WEvent::KeyboardInput { input, .. } => {

							if let Some(kc) = input.virtual_keycode {

								if let Some(key) = Key::from_winit(kc) {

									match input.state {

										ElementState::Pressed => {

											events.push(Event::KeyPressRepeat(key));

											if !self.key_down(key) {
												events.push(Event::KeyPress(key));
											}

											self.pressed_keys.insert(key);

										},

										ElementState::Released => {
											self.pressed_keys.remove(&key);
											events.push(Event::KeyRelease(key));
										},

									}

								}

							}

						},

						WEvent::MouseInput { button, state, .. } => {

							if let Some(button) = Mouse::from_winit(*button) {

								match state {

									ElementState::Pressed => {
										self.pressed_mouse.insert(button);
										events.push(Event::MousePress(button));
									},
									ElementState::Released => {
										self.pressed_mouse.remove(&button);
										events.push(Event::MouseRelease(button));
									},

								}

							}

						},

						WEvent::CursorMoved { position, .. } => {

							let mpos: Vec2 = position.to_logical(self.dpi() as f64).into();
							let (w, h) = (self.width as f32, self.height as f32);
							let mpos = vec2!(mpos.x - w / 2.0, h / 2.0 - mpos.y);

							self.mouse_pos = mpos;

						},

						WEvent::MouseWheel { delta, phase, .. } => {

							match phase {
								TouchPhase::Started => {
									self.scroll_phase = ScrollPhase::Solid;
								},
								TouchPhase::Ended => {
									self.scroll_phase = ScrollPhase::Trailing;
								},
								_ => {},
							}

							let p = self.scroll_phase;
							let d: Vec2 = (*delta).into();

							events.push(Event::Wheel(vec2!(d.x, -d.y), p));

						},

						WEvent::ReceivedCharacter(ch) => {
							if !INVALID_CHARS.contains(&ch) && !ch.is_control() {
								events.push(Event::CharInput(*ch));
							}
						},

						WEvent::Resized(size) => {

							let dpi = self.dpi() as f64;
							let lsize: LogicalSize<f64> = size.to_logical(dpi);
							let w = lsize.width as i32;
							let h = lsize.height as i32;

							self.width = w;
							self.height = h;
							self.windowed_ctx.resize(*size);

							handle(&mut self, WindowEvent::Resize(w, h))?;
							events.push(Event::Resize(w, h));

						},

						WEvent::Touch(touch) => {
							events.push(Event::Touch(touch.id, touch.location.into()));
						},

						WEvent::HoveredFile(path) => {
							events.push(Event::FileHover(path.to_path_buf()));
						},

						WEvent::HoveredFileCancelled => {
							events.push(Event::FileHoverCancel);
						},

						WEvent::DroppedFile(path) => {
							events.push(Event::FileDrop(path.to_path_buf()));
						},

						WEvent::Focused(b) => {
							events.push(Event::Focus(*b));
						},

						WEvent::CursorEntered { .. } => {
							events.push(Event::CursorEnter);
						},

						WEvent::CursorLeft { .. } => {
							events.push(Event::CursorLeave);
						},

						_ => (),

					},

					glutin::event::Event::DeviceEvent { event, .. } => match event {
						DEvent::MouseMotion { delta } => {
							events.push(Event::MouseMove(vec2!(delta.0, -delta.1)));
						},
						_ => (),
					},

					glutin::event::Event::RedrawRequested(_) => {

						handle(&mut self, WindowEvent::Frame)?;
						self.swap()?;

					},

					glutin::event::Event::MainEventsCleared => {

						// ugly workaround
						update = !update;

						if update {
							self.windowed_ctx
								.window()
								.request_redraw();
						}

						while let Some(gilrs::Event { id, event, .. }) = self.gamepad_ctx.next_event() {

							use gilrs::ev::EventType::*;

							match event {

								ButtonPressed(button, ..) => {

									if let Some(button) = GamepadButton::from_gilrs(button) {

										self
											.pressed_gamepad_buttons
											.entry(id)
											.or_insert(hset![])
											.insert(button);

										events.push(Event::GamepadPress(id, button));

									}

								},

								ButtonRepeated(button, ..) => {
									if let Some(button) = GamepadButton::from_gilrs(button) {
										events.push(Event::GamepadPressRepeat(id, button));
									}
								},

								ButtonReleased(button, ..) => {

									if let Some(button) = GamepadButton::from_gilrs(button) {

										self
											.pressed_gamepad_buttons
											.entry(id)
											.or_insert(hset![])
											.remove(&button);

										events.push(Event::GamepadRelease(id, button));

									}

								},

								AxisChanged(axis, val, ..) => {

									let mut pos = self.gamepad_axis_pos
										.entry(id)
										.or_insert((vec2!(), vec2!()))
										.clone()
										;

									match axis {
										gilrs::ev::Axis::LeftStickX => {
											pos.0.x = val;
											events.push(Event::GamepadAxis(id, GamepadAxis::LStick, pos.0));
										},
										gilrs::ev::Axis::LeftStickY => {
											pos.0.y = val;
											events.push(Event::GamepadAxis(id, GamepadAxis::LStick, pos.0));
										},
										gilrs::ev::Axis::RightStickX => {
											pos.1.x = val;
											events.push(Event::GamepadAxis(id, GamepadAxis::RStick, pos.1));
										},
										gilrs::ev::Axis::RightStickY => {
											pos.1.y = val;
											events.push(Event::GamepadAxis(id, GamepadAxis::RStick, pos.1));
										},
										_ => {},

									}

									self.gamepad_axis_pos.insert(id, pos);

								},

								Connected => {
									events.push(Event::GamepadConnect(id));
								},

								Disconnected => {
									events.push(Event::GamepadDisconnect(id));
								},

								_ => {},

							}

						}

					},

					_ => {},

				};

				for e in events {
					handle(&mut self, WindowEvent::Input(e))?;
				}

			};

			if let Err(err) = res {
				elog!("{}", err);
			}

		});

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

}

impl From<glutin::event::MouseScrollDelta> for Vec2 {
	fn from(delta: glutin::event::MouseScrollDelta) -> Self {
		use glutin::event::MouseScrollDelta;
		match delta {
			MouseScrollDelta::PixelDelta(pos) => {
				return vec2!(pos.x, pos.y);
			},
			MouseScrollDelta::LineDelta(x, y) => {
				return vec2!(x, y);
			}
		};
	}
}

impl From<Vec2> for LogicalPosition<f64> {
	fn from(pos: Vec2) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

impl From<LogicalPosition<f64>> for Vec2 {
	fn from(pos: LogicalPosition<f64>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

#[cfg(not(web))]
impl From<PhysicalPosition<f64>> for Vec2 {
	fn from(pos: PhysicalPosition<f64>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

impl From<PhysicalPosition<i32>> for Vec2 {
	fn from(pos: PhysicalPosition<i32>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

impl CursorIcon {
	fn to_winit(&self) -> glutin::window::CursorIcon {
		return match self {
			CursorIcon::Normal => glutin::window::CursorIcon::Default,
			CursorIcon::Hand => glutin::window::CursorIcon::Hand,
			CursorIcon::Cross => glutin::window::CursorIcon::Crosshair,
			CursorIcon::Move => glutin::window::CursorIcon::Move,
			CursorIcon::Progress => glutin::window::CursorIcon::Progress,
			CursorIcon::Wait => glutin::window::CursorIcon::Wait,
			CursorIcon::Text => glutin::window::CursorIcon::Text,
		};
	}
}

impl Key {

	fn from_winit(k: glutin::event::VirtualKeyCode) -> Option<Self> {
		use glutin::event::VirtualKeyCode::*;
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
			Key1 => Some(Key::Key1),
			Key2 => Some(Key::Key2),
			Key3 => Some(Key::Key3),
			Key4 => Some(Key::Key4),
			Key5 => Some(Key::Key5),
			Key6 => Some(Key::Key6),
			Key7 => Some(Key::Key7),
			Key8 => Some(Key::Key8),
			Key9 => Some(Key::Key9),
			Key0 => Some(Key::Key0),
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
			Grave => Some(Key::Backquote),
			Slash => Some(Key::Slash),
			Backslash => Some(Key::Backslash),
			Semicolon => Some(Key::Semicolon),
			Apostrophe => Some(Key::Quote),
			Up => Some(Key::Up),
			Down => Some(Key::Down),
			Left => Some(Key::Left),
			Right => Some(Key::Right),
			Escape => Some(Key::Esc),
			Tab => Some(Key::Tab),
			Space => Some(Key::Space),
			Back => Some(Key::Backspace),
			Return => Some(Key::Enter),
			LShift => Some(Key::LShift),
			RShift => Some(Key::RShift),
			LAlt => Some(Key::LAlt),
			RAlt => Some(Key::RAlt),
			LWin => Some(Key::LMeta),
			RWin => Some(Key::RMeta),
			LControl => Some(Key::LCtrl),
			RControl => Some(Key::RCtrl),
			_ => None,
		};
	}

}

impl Mouse {
	fn from_winit(m: glutin::event::MouseButton) -> Option<Self> {
		use glutin::event::MouseButton::*;
		return match m {
			Left => Some(Mouse::Left),
			Right => Some(Mouse::Right),
			Middle => Some(Mouse::Middle),
			_ => None,
		};
	}
}

impl GamepadButton {
	fn from_gilrs(b: gilrs::ev::Button) -> Option<Self> {
		use gilrs::ev::Button::*;
		return match b {
			South => Some(GamepadButton::A),
			East => Some(GamepadButton::B),
			West => Some(GamepadButton::X),
			North => Some(GamepadButton::Y),
			LeftTrigger => Some(GamepadButton::LBumper),
			LeftTrigger2 => Some(GamepadButton::LTrigger),
			RightTrigger => Some(GamepadButton::RBumper),
			RightTrigger2 => Some(GamepadButton::RTrigger),
			Select => Some(GamepadButton::Select),
			Start => Some(GamepadButton::Start),
			Mode => Some(GamepadButton::Mode),
			LeftThumb => Some(GamepadButton::LStick),
			RightThumb => Some(GamepadButton::RStick),
			DPadUp => Some(GamepadButton::Up),
			DPadDown => Some(GamepadButton::Down),
			DPadLeft => Some(GamepadButton::Left),
			DPadRight => Some(GamepadButton::Right),
			_ => None,
		};
	}
}

