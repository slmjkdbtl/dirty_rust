// wengwengweng

use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
pub use gilrs::GamepadId as GamepadID;
#[cfg(target_arch = "wasm32")]
pub type GamepadID = u64;

pub type TouchID = u64;

use super::*;
use crate::*;
use window::Pos;

pub enum Event {
	KeyDown(Key),
	KeyPress(Key),
	KeyPressRepeat(Key),
	KeyRelease(Key),
	MouseDown(Mouse),
	MousePress(Mouse),
	MouseRelease(Mouse),
	MouseMove(Pos),
	Scroll(Pos),
	TextInput(char),
	GamepadPress(GamepadID, GamepadButton),
	GamepadPressRepeat(GamepadID, GamepadButton),
	GamepadRelease(GamepadID, GamepadButton),
	GamepadDown(GamepadID, GamepadButton),
	GamepadAxis(GamepadID, GamepadAxis, Vec2),
	GamepadConnect(GamepadID),
	GamepadDisconnect(GamepadID),
	Touch(TouchID, Pos),
	Resize(u32, u32),
	FileHover(PathBuf),
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

fn key_down(ctx: &app::Ctx, key: Key) -> bool {
	return ctx.key_states.get(&key) == Some(&ButtonState::Down) || key_pressed(ctx, key);
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

fn mouse_down(ctx: &app::Ctx, mouse: Mouse) -> bool {
	return ctx.mouse_states.get(&mouse) == Some(&ButtonState::Down) || mouse_pressed(ctx, mouse);
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

fn gamepad_released(ctx: &app::Ctx, id: GamepadID, button: GamepadButton) -> bool {
	if let Some(states) = ctx.gamepad_button_states.get(&id) {
		return states.get(&button) == Some(&ButtonState::Released);
	} else {
		return false;
	}
}

fn gamepad_down(ctx: &app::Ctx, id: GamepadID, button: GamepadButton) -> bool {
	if let Some(states) = ctx.gamepad_button_states.get(&id) {
		return states.get(&button) == Some(&ButtonState::Down) || gamepad_pressed(ctx, id, button);
	} else {
		return false;
	}
}

fn gamepad_pressed(ctx: &app::Ctx, id: GamepadID, button: GamepadButton) -> bool {
	if let Some(states) = ctx.gamepad_button_states.get(&id) {
		return states.get(&button) == Some(&ButtonState::Pressed);
	} else {
		return false;
	}
}

#[cfg(target_arch = "wasm32")]
pub(super) fn poll(ctx: &mut app::Ctx) -> Result<Vec<Event>> {
	return Ok(vec![]);
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn poll(ctx: &mut app::Ctx) -> Result<Vec<Event>> {

	let mut events = vec![];

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

	let mut keyboard_input = None;
	let mut mouse_input = None;
	let mut cursor_moved = None;
	let mut resized = None;
	let mut close = false;

	ctx.events_loop.poll_events(|e| {

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
					events.push(Event::Scroll(delta.into()));
				},

				ReceivedCharacter(ch) => {
					events.push(Event::TextInput(ch));
				},

				Resized(size) => {
					resized = Some(size);
				},

				Touch(touch) => {
					events.push(Event::Touch(touch.id, touch.location.into()));
				},

				HoveredFile(path) => {
					events.push(Event::FileHover(path));
				},

				DroppedFile(path) => {
					events.push(Event::FileDrop(path));
				},

				Focused(b) => {
					events.push(Event::Focus(b));
				},

				CloseRequested => close = true,

				_ => {},

			},

			DeviceEvent { event, .. } => match event {
				glutin::DeviceEvent::MouseMotion { delta } => {
					events.push(Event::MouseMove(Pos {
						x: delta.0 as i32,
						y: delta.1 as i32,
					}));
				},
				_ => (),
			},

			_ => {},

		};

	});

	if close {
		ctx.quit = true;
		return Ok(events);
	}

	if let Some(input) = keyboard_input {

		if let Some(kc) = input.virtual_keycode {

			if let Some(key) = Key::from_extern(kc) {

				match input.state {

					glutin::ElementState::Pressed => {

						events.push(Event::KeyPressRepeat(key));

						if key_up(ctx, key) || key_released(ctx, key) {
							ctx.key_states.insert(key, ButtonState::Pressed);
							events.push(Event::KeyPress(key));
						}

					},

					glutin::ElementState::Released => {
						if key_down(ctx, key) || key_pressed(ctx, key) {
							ctx.key_states.insert(key, ButtonState::Released);
							events.push(Event::KeyRelease(key));
						}
					},

				}

			}

		}

	}

	if let Some((button, state)) = mouse_input {

		if let Some(button) = Mouse::from_extern(button) {

			match state {

				glutin::ElementState::Pressed => {
					if mouse_up(ctx, button) || mouse_released(ctx, button) {
						ctx.mouse_states.insert(button, ButtonState::Pressed);
						events.push(Event::MousePress(button));
					}
				},
				glutin::ElementState::Released => {
					if mouse_down(ctx, button) || mouse_pressed(ctx, button) {
						ctx.mouse_states.insert(button, ButtonState::Released);
						events.push(Event::MouseRelease(button));
					}
				},

			}

		}

	}

	if let Some(pos) = cursor_moved {
		ctx.mouse_pos = pos.into();
	}

	if let Some(size) = resized {
		ctx.width = size.width as i32;
		ctx.height = size.height as i32;
		events.push(Event::Resize(ctx.width as u32, ctx.height as u32));
	}

	ctx.key_states
		.iter()
		.filter(|(_, &state)| state == ButtonState::Down || state == ButtonState::Pressed)
		.for_each(|(&k, _)| {
			events.push(Event::KeyDown(k));
		});

	#[cfg(all(not(target_os = "ios"), not(target_os = "android"), not(target_arch = "wasm32")))]
	while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad_ctx.next_event() {

		use gilrs::ev::EventType::*;

		// TODO: GamepadDown
		match event {

			ButtonPressed(button, ..) => {

				if let Some(button) = GamepadButton::from_extern(button) {

					if gamepad_up(ctx, id, button) || gamepad_released(ctx, id, button) {

						ctx
							.gamepad_button_states
							.entry(id)
							.or_insert(HashMap::new())
							.insert(button, ButtonState::Pressed);

						events.push(Event::GamepadPress(id, button));

					}

				}

			},

			ButtonRepeated(button, ..) => {
				if let Some(button) = GamepadButton::from_extern(button) {
					events.push(Event::GamepadPressRepeat(id, button));
				}
			},

			ButtonReleased(button, ..) => {
				if let Some(button) = GamepadButton::from_extern(button) {

					if gamepad_down(ctx, id, button) || gamepad_pressed(ctx, id, button) {

						ctx
							.gamepad_button_states
							.entry(id)
							.or_insert(HashMap::new())
							.insert(button, ButtonState::Released);

						events.push(Event::GamepadRelease(id, button));

					}

				}

			},

			AxisChanged(axis, val, ..) => {

				let mut pos = ctx.gamepad_axis_pos.entry(id).or_insert((vec2!(), vec2!()));

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

	for (id, states) in &ctx.gamepad_button_states {
		states
			.iter()
			.filter(|(_, &state)| state == ButtonState::Down || state == ButtonState::Pressed)
			.for_each(|(&k, _)| {
				events.push(Event::GamepadDown(*id, k));
			});
	}

	return Ok(events);

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

		impl $type {

			fn from_str(s: &str) -> Option<Self> {

				return match s {
					$(
						$str => Some($type::$name),
					)*
					_ => None,
				};

			}

			#[cfg(not(target_arch = "wasm32"))]
			fn from_extern(s: $xtype) -> Option<Self> {
				return match s {
					$(
						<$xtype>::$xname => Some($type::$name),
					)*
					_ => None,
				};
			}

			fn as_str(&self) -> &'static str {
				return match self {
					$(
						$type::$name => $str,
					)*
				};
			}

			#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
use glutin::VirtualKeyCode as ExternKey;
#[cfg(not(target_arch = "wasm32"))]
use glutin::MouseButton as ExternMouse;
#[cfg(not(target_arch = "wasm32"))]
use gilrs::ev::Button as ExternGamepadButton;

#[cfg(target_arch = "wasm32")]
struct ExternKey;
#[cfg(target_arch = "wasm32")]
struct ExternMouse;
#[cfg(target_arch = "wasm32")]
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
	A("a") => South,
	B("b") => East,
	X("x") => West,
	Y("y") => North,
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GamepadAxis {
	LStick,
	RStick,
}

