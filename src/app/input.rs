// wengwengweng

//! Input & Events

use std::path::PathBuf;
use std::collections::HashSet;

use once_cell::sync::Lazy;

static INVALID_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
	return hashset![
		'\u{7f}',
		'\r',
		'\n',
		'\u{1b}',
		'\u{8}',
		'\u{f700}',
		'\u{f701}',
		'\u{f702}',
		'\u{f703}',
		'\u{f704}',
		'\u{f705}',
		'\u{f706}',
		'\u{f707}',
		'\u{f708}',
		'\u{f709}',
		'\u{f70a}',
		'\u{f70b}',
		'\u{f70c}',
		'\u{f70d}',
		'\u{f70e}',
		'\u{f70f}',
	];
});

#[cfg(desktop)]
pub use gilrs::GamepadId as GamepadID;
#[cfg(any(mobile, web))]
pub type GamepadID = u64;

pub type TouchID = u64;

use super::*;
use crate::*;

impl Ctx {

	pub fn down_keys(&self) -> HashSet<Key> {

		use ButtonState::*;

		return self.key_states
			.iter()
			.filter(|(_, &state)| state == Down || state == Pressed)
			.map(|(key, _)| *key)
			.collect();

	}

	pub fn key_down(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Down) || key_pressed(self, key);
	}

	pub fn mouse_down(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Down) || mouse_pressed(self, mouse);
	}

	pub fn gamepad_down(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(states) = self.gamepad_button_states.get(&id) {
			return states.get(&button) == Some(&ButtonState::Down) || gamepad_pressed(self, id, button);
		} else {
			return false;
		}
	}

	pub fn mouse_pos(&self) -> Vec2 {

		let (w, h) = (self.width, self.height);
		let (gw, gh) = (self.gwidth(), self.gheight());
		let x = self.mouse_pos.x * gw as f32 / w as f32;
		let y = self.mouse_pos.y * gh as f32 / h as f32;

		return vec2!(x, y);

	}

}

fn key_pressed(ctx: &app::Ctx, key: Key) -> bool {
	return ctx.key_states.get(&key) == Some(&ButtonState::Pressed);
}

fn key_released(ctx: &app::Ctx, key: Key) -> bool {
	return ctx.key_states.get(&key) == Some(&ButtonState::Released);
}

fn key_up(ctx: &app::Ctx, key: Key) -> bool {
	return ctx.key_states.get(&key) == Some(&ButtonState::Up) || ctx.key_states.get(&key).is_none();
}

fn mouse_pressed(ctx: &app::Ctx, mouse: Mouse) -> bool {
	return ctx.mouse_states.get(&mouse) == Some(&ButtonState::Pressed);
}

fn mouse_released(ctx: &app::Ctx, mouse: Mouse) -> bool {
	return ctx.mouse_states.get(&mouse) == Some(&ButtonState::Released);
}

fn mouse_up(ctx: &app::Ctx, mouse: Mouse) -> bool {
	return ctx.mouse_states.get(&mouse) == Some(&ButtonState::Up) || ctx.mouse_states.get(&mouse).is_none();
}

fn gamepad_up(ctx: &app::Ctx, id: GamepadID, button: GamepadButton) -> bool {
	if let Some(states) = ctx.gamepad_button_states.get(&id) {
		return states.get(&button) == Some(&ButtonState::Up) || states.get(&button).is_none();
	} else {
		return true;
	}
}

fn gamepad_pressed(ctx: &app::Ctx, id: GamepadID, button: GamepadButton) -> bool {
	if let Some(states) = ctx.gamepad_button_states.get(&id) {
		return states.get(&button) == Some(&ButtonState::Pressed);
	} else {
		return false;
	}
}

fn gamepad_released(ctx: &app::Ctx, id: GamepadID, button: GamepadButton) -> bool {
	if let Some(states) = ctx.gamepad_button_states.get(&id) {
		return states.get(&button) == Some(&ButtonState::Released);
	} else {
		return false;
	}
}

#[derive(Clone, Debug)]
pub enum Event {
	KeyPress(Key),
	KeyPressRepeat(Key),
	KeyRelease(Key),
	MousePress(Mouse),
	MouseRelease(Mouse),
	MouseMove(Vec2),
	Scroll(Vec2, ScrollPhase),
	TextInput(char),
	GamepadPress(GamepadID, GamepadButton),
	GamepadPressRepeat(GamepadID, GamepadButton),
	GamepadRelease(GamepadID, GamepadButton),
	GamepadAxis(GamepadID, GamepadAxis, Vec2),
	GamepadConnect(GamepadID),
	GamepadDisconnect(GamepadID),
	Touch(TouchID, Vec2),
	Resize(i32, i32),
	FileHover(PathBuf),
	FileHoverCancel,
	FileDrop(PathBuf),
	Focus(bool),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

#[cfg(web)]
pub(super) fn poll(ctx: &mut app::Ctx) -> Result<()> {
	return Ok(());
}

#[cfg(not(web))]
pub(super) fn poll(
	mut ctx: &mut app::Ctx,
	events_loop: &mut glutin::EventsLoop,
	s: &mut impl app::State,
) -> Result<()> {

	for state in ctx.key_states.values_mut() {
		if state == &ButtonState::Pressed {
			*state = ButtonState::Down;
		} else if state == &ButtonState::Released {
			*state = ButtonState::Up;
		}
	}

	for state in ctx.mouse_states.values_mut() {
		if state == &ButtonState::Pressed {
			*state = ButtonState::Down;
		} else if state == &ButtonState::Released {
			*state = ButtonState::Up;
		}
	}

	for states in ctx.gamepad_button_states.values_mut() {
		for state in states.values_mut() {
			if state == &ButtonState::Pressed {
				*state = ButtonState::Down;
			} else if state == &ButtonState::Released {
				*state = ButtonState::Up;
			}
		}
	}

	#[cfg(feature = "imgui")]
	let mut imgui_events = vec![];

	let mut res: Result<()> = Ok(());
	let mut close = false;

	events_loop.poll_events(|e| {

		res = try {

			use glutin::Event::*;
			use glutin::WindowEvent::*;
			use glutin::TouchPhase;

			#[cfg(feature = "imgui")]
			imgui_events.push(e.clone());

			match e {

				WindowEvent { event, .. } => match event {

					KeyboardInput { input, .. } => {

						if let Some(kc) = input.virtual_keycode {

							if let Some(key) = Key::from_extern(kc) {

								match input.state {

									glutin::ElementState::Pressed => {

										s.event(&mut ctx, &Event::KeyPressRepeat(key))?;

										if key_up(ctx, key) || key_released(ctx, key) {
											ctx.key_states.insert(key, ButtonState::Pressed);
											s.event(&mut ctx, &Event::KeyPress(key))?;
										}

									},

									glutin::ElementState::Released => {
										if ctx.key_down(key) || key_pressed(ctx, key) {
											ctx.key_states.insert(key, ButtonState::Released);
											s.event(&mut ctx, &Event::KeyRelease(key))?;
										}
									},

								}

							}

						}

					},

					MouseInput { button, state, .. } => {

						if let Some(button) = Mouse::from_extern(button) {

							match state {

								glutin::ElementState::Pressed => {
									if mouse_up(ctx, button) || mouse_released(ctx, button) {
										ctx.mouse_states.insert(button, ButtonState::Pressed);
										s.event(&mut ctx, &Event::MousePress(button))?;
									}
								},
								glutin::ElementState::Released => {
									if ctx.mouse_down(button) || mouse_pressed(ctx, button) {
										ctx.mouse_states.insert(button, ButtonState::Released);
										s.event(&mut ctx, &Event::MouseRelease(button))?;
									}
								},

							}

						}

					},

					CursorMoved { position, .. } => {

						let offset = (ctx.conf.origin.as_pt() / 2.0 + vec2!(0.5)) * vec2!(ctx.width(), ctx.height());
						let mpos: Vec2 = position.into();

						ctx.mouse_pos = mpos - offset;

					},

					MouseWheel { delta, phase, .. } => {

						match phase {
							TouchPhase::Started => {
								ctx.scroll_phase = ScrollPhase::Solid;
							},
							TouchPhase::Ended => {
								ctx.scroll_phase = ScrollPhase::Trailing;
							},
							_ => {},
						}

						let p = ctx.scroll_phase;
						s.event(&mut ctx, &Event::Scroll(delta.into(), p))?;

					},

					ReceivedCharacter(ch) => {
						if !INVALID_CHARS.contains(&ch) {
							s.event(&mut ctx, &Event::TextInput(ch))?;
						}
					},

					Resized(size) => {

						let w = size.width as i32;
						let h = size.height as i32;

						ctx.width = w;
						ctx.height = h;

						s.event(&mut ctx, &Event::Resize(w, h))?;

					},

					Touch(touch) => {
						s.event(&mut ctx, &Event::Touch(touch.id, touch.location.into()))?;
					},

					HoveredFile(path) => {
						s.event(&mut ctx, &Event::FileHover(path))?;
					},

					HoveredFileCancelled => {
						s.event(&mut ctx, &Event::FileHoverCancel)?;
					},

					DroppedFile(path) => {
						s.event(&mut ctx, &Event::FileDrop(path))?;
					},

					Focused(b) => {
						s.event(&mut ctx, &Event::Focus(b))?;
					},

					CloseRequested => close = true,

					_ => {},

				},

				DeviceEvent { event, .. } => match event {
					glutin::DeviceEvent::MouseMotion { delta } => {
						let scale = ctx.conf.scale;
						s.event(&mut ctx, &Event::MouseMove(vec2!(delta.0, delta.1) / scale))?;
					},
					_ => (),
				},

				_ => {},

			};

		};

	});

	res?;

	#[cfg(feature = "imgui")]
	for e in imgui_events {
		ctx.imgui_platform.handle_event(ctx.imgui_ctx.io_mut(), &ctx.windowed_ctx.window(), &e);
	}

	if close {
		ctx.quit = true;
		return Ok(());
	}

	#[cfg(desktop)]
	while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad_ctx.next_event() {

		use gilrs::ev::EventType::*;

		match event {

			ButtonPressed(button, ..) => {

				if let Some(button) = GamepadButton::from_extern(button) {

					if gamepad_up(ctx, id, button) || gamepad_released(ctx, id, button) {

						ctx
							.gamepad_button_states
							.entry(id)
							.or_insert(HashMap::new())
							.insert(button, ButtonState::Pressed);

						s.event(&mut ctx, &Event::GamepadPress(id, button))?;

					}

				}

			},

			ButtonRepeated(button, ..) => {
				if let Some(button) = GamepadButton::from_extern(button) {
					s.event(&mut ctx, &Event::GamepadPressRepeat(id, button))?;
				}
			},

			ButtonReleased(button, ..) => {
				if let Some(button) = GamepadButton::from_extern(button) {

					if ctx.gamepad_down(id, button) || gamepad_pressed(ctx, id, button) {

						ctx
							.gamepad_button_states
							.entry(id)
							.or_insert(HashMap::new())
							.insert(button, ButtonState::Released);

						s.event(&mut ctx, &Event::GamepadRelease(id, button))?;

					}

				}

			},

			AxisChanged(axis, val, ..) => {

				let mut pos = ctx.gamepad_axis_pos
					.entry(id)
					.or_insert((vec2!(), vec2!()))
					.clone()
					;

				match axis {
					gilrs::ev::Axis::LeftStickX => {
						pos.0.x = val;
						s.event(&mut ctx, &Event::GamepadAxis(id, GamepadAxis::LStick, pos.0))?;
					},
					gilrs::ev::Axis::LeftStickY => {
						pos.0.y = val;
						s.event(&mut ctx, &Event::GamepadAxis(id, GamepadAxis::LStick, pos.0))?;
					},
					gilrs::ev::Axis::RightStickX => {
						pos.1.x = val;
						s.event(&mut ctx, &Event::GamepadAxis(id, GamepadAxis::RStick, pos.1))?;
					},
					gilrs::ev::Axis::RightStickY => {
						pos.1.y = val;
						s.event(&mut ctx, &Event::GamepadAxis(id, GamepadAxis::RStick, pos.1))?;
					},
					_ => {},

				}

				ctx.gamepad_axis_pos.insert(id, pos);

			},

			Connected => {
				s.event(&mut ctx, &Event::GamepadConnect(id))?;
			},

			Disconnected => {
				s.event(&mut ctx, &Event::GamepadDisconnect(id))?;
			},

			_ => {},

		}

	}

	return Ok(());

}

macro_rules! gen_buttons {

	// TODO: generate const str to replace $str
	($type:ident($xtype:ty), {$($name:ident($str:expr) => $xname:ident),*$(,)?}) => {

		#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
		pub enum $type {
			$(
				$name,
			)*
		}

		impl std::str::FromStr for $type {

			type Err = ();

			fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
				return match s {
					$(
						$str => Ok($type::$name),
					)*
					_ => Err(()),
				};
			}

		}

		impl $type {

			#[allow(dead_code)]
			#[cfg(desktop)]
			fn from_extern(s: $xtype) -> Option<Self> {
				return match s {
					$(
						<$xtype>::$xname => Some($type::$name),
					)*
					_ => None,
				};
			}

			// TODO
			#[allow(dead_code)]
			#[cfg(not(desktop))]
			fn from_extern(s: $xtype) -> Option<Self> {
				return None;
			}

			#[allow(dead_code)]
			fn as_str(&self) -> &'static str {
				return match self {
					$(
						$type::$name => $str,
					)*
				};
			}

			#[allow(dead_code)]
			#[cfg(desktop)]
			fn as_extern(&self) -> $xtype {
				return match self {
					$(
						$type::$name => <$xtype>::$xname,
					)*
				};
			}

		}

	}

}

#[cfg(not(web))]
use glutin::VirtualKeyCode as ExternKey;
#[cfg(not(web))]
use glutin::MouseButton as ExternMouse;
#[cfg(web)]
struct ExternKey;
#[cfg(web)]
struct ExternMouse;

#[cfg(desktop)]
use gilrs::ev::Button as ExternGamepadButton;
#[cfg(any(mobile, web))]
struct ExternGamepadButton;

gen_buttons!(Key(ExternKey), {
	Q("q") => Q,
	W("w") => W,
	E("e") => E,
	R("r") => R,
	T("t") => T,
	Y("y") => Y,
	U("u") => U,
	I("i") => I,
	O("o") => O,
	P("p") => P,
	A("a") => A,
	S("s") => S,
	D("d") => D,
	F("f") => F,
	G("g") => G,
	H("h") => H,
	J("j") => J,
	K("k") => K,
	L("l") => L,
	Z("z") => Z,
	X("x") => X,
	C("c") => C,
	V("v") => V,
	B("b") => B,
	N("n") => N,
	M("m") => M,
	Key1("1") => Key1,
	Key2("2") => Key2,
	Key3("3") => Key3,
	Key4("4") => Key4,
	Key5("5") => Key5,
	Key6("6") => Key6,
	Key7("7") => Key7,
	Key8("8") => Key8,
	Key9("9") => Key9,
	Key0("0") => Key0,
	F1("f1") => F1,
	F2("f2") => F2,
	F3("f3") => F3,
	F4("f4") => F4,
	F5("f5") => F5,
	F6("f6") => F6,
	F7("f7") => F7,
	F8("f8") => F8,
	F9("f9") => F9,
	F10("f10") => F10,
	F11("f11") => F11,
	F12("f12") => F12,
	Minus("-") => Minus,
	Equals("=") => Equals,
	Comma(",") => Comma,
	Period(".") => Period,
	Grave("`") => Grave,
	Slash("/") => Slash,
	Backslash("\\") => Backslash,
	Semicolon(";") => Semicolon,
	Apostrophe("'") => Apostrophe,
	Up("up") => Up,
	Down("down") => Down,
	Left("left") => Left,
	Right("right") => Right,
	Esc("esc") => Escape,
	Tab("tab") => Tab,
	Space("space") => Space,
	Back("back") => Back,
	Return("return") => Return,
	LShift("lshift") => LShift,
	RShift("rshift") => RShift,
	LAlt("lalt") => LAlt,
	RAlt("ralt") => RAlt,
	LWin("lwin") => LWin,
	RWin("rwin") => RWin,
	LCtrl("lctrl") => LControl,
	RCtrl("rctrl") => RControl,
});

gen_buttons!(Mouse(ExternMouse), {
	Left("left") => Left,
	Right("right") => Right,
	Middle("middle") => Middle,
});

gen_buttons!(GamepadButton(ExternGamepadButton), {
	South("south") => South,
	East("east") => East,
	West("west") => West,
	North("north") => North,
	LBumper("lbumper") => LeftTrigger,
	LTrigger("ltrigger") => LeftTrigger2,
	RBumper("rbumper") => RightTrigger,
	RTrigger("rtrigger") => RightTrigger2,
	Select("select") => Select,
	Start("start") => Start,
	Mode("mode") => Mode,
	LStick("lstick") => LeftThumb,
	RStick("rstick") => RightThumb,
	Up("up") => DPadUp,
	Down("down") => DPadDown,
	Left("left") => DPadLeft,
	Right("right") => DPadRight,
});

#[derive(Debug, Clone, Copy)]
pub enum ScrollPhase {
	Solid,
	Trailing,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GamepadAxis {
	LStick,
	RStick,
}

