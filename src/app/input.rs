// wengwengweng

#[cfg(not(target_arch = "wasm32"))]
use glutin::ElementState;
#[cfg(not(target_arch = "wasm32"))]
pub use glutin::ModifiersState as Mod;
#[cfg(not(target_arch = "wasm32"))]
pub use glutin::VirtualKeyCode as Key;
#[cfg(not(target_arch = "wasm32"))]
pub use glutin::MouseButton as Mouse;
#[cfg(not(target_arch = "wasm32"))]
pub use gilrs::ev::Button as GamepadButton;
#[cfg(not(target_arch = "wasm32"))]
pub use gilrs::ev::Axis as GamepadAxis;
#[cfg(not(target_arch = "wasm32"))]
pub use gilrs::GamepadId as GamepadID;

// TODO: input types for browser
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mod;
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key;
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mouse;

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
	TextInput(String),
	GamepadPress(GamepadID, GamepadButton),
	GamepadPressRepeat(GamepadID, GamepadButton),
	GamepadRelease(GamepadID, GamepadButton),
	GamepadDown(GamepadID, GamepadButton),
	GamepadAxis(GamepadID, GamepadAxis, f32),
	GamepadConnect(GamepadID),
	GamepadDisconnect(GamepadID),
	Resize(u32, u32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

fn key_down(ctx: &app::Ctx, key: Key) -> bool {
	return ctx.key_state.get(&key) == Some(&ButtonState::Down) || key_pressed(ctx, key);
}

fn key_pressed(ctx: &app::Ctx, key: Key) -> bool {
	return ctx.key_state.get(&key) == Some(&ButtonState::Pressed);
}

fn key_released(ctx: &app::Ctx, key: Key) -> bool {
	return ctx.key_state.get(&key) == Some(&ButtonState::Released);
}

fn key_up(ctx: &app::Ctx, key: Key) -> bool {
	return ctx.key_state.get(&key) == Some(&ButtonState::Up) || ctx.key_state.get(&key).is_none();
}

fn mouse_down(ctx: &app::Ctx, mouse: Mouse) -> bool {
	return ctx.mouse_state.get(&mouse) == Some(&ButtonState::Down) || mouse_pressed(ctx, mouse);
}

fn mouse_pressed(ctx: &app::Ctx, mouse: Mouse) -> bool {
	return ctx.mouse_state.get(&mouse) == Some(&ButtonState::Pressed);
}

fn mouse_released(ctx: &app::Ctx, mouse: Mouse) -> bool {
	return ctx.mouse_state.get(&mouse) == Some(&ButtonState::Released);
}

fn mouse_up(ctx: &app::Ctx, mouse: Mouse) -> bool {
	return ctx.mouse_state.get(&mouse) == Some(&ButtonState::Up) || ctx.mouse_state.get(&mouse).is_none();
}

pub(super) fn poll(ctx: &mut app::Ctx) -> Result<Vec<Event>> {

	let mut events = vec![];

	for state in ctx.key_state.values_mut() {
		if state == &ButtonState::Pressed {
			*state = ButtonState::Down;
		} else if state == &ButtonState::Released {
			*state = ButtonState::Up;
		}
	}

	for state in ctx.mouse_state.values_mut() {
		if state == &ButtonState::Pressed {
			*state = ButtonState::Down;
		} else if state == &ButtonState::Released {
			*state = ButtonState::Up;
		}
	}

	let mut keyboard_input = None;
	let mut mouse_input = None;
	let mut cursor_moved = None;
	let mut mouse_wheel = None;
	let mut mouse_delta = None;
	let mut text_input = None;
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
					mouse_wheel = Some(delta);
				},

				ReceivedCharacter(ch) => {
					text_input.get_or_insert(String::new()).push(ch);
				},

				Resized(size) => {
					resized = Some(size);
				},

				Touch(touch) => {
					// ...
				},

				CloseRequested => close = true,

				_ => {},

			},

			DeviceEvent { event, .. } => match event {
				glutin::DeviceEvent::MouseMotion { delta } => {
					mouse_delta = Some(delta);
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

			match input.state {

				ElementState::Pressed => {

					events.push(Event::KeyPressRepeat(kc));

					if key_up(ctx, kc) || key_released(ctx, kc) {
						ctx.key_state.insert(kc, ButtonState::Pressed);
						events.push(Event::KeyPress(kc));
					}

				},

				ElementState::Released => {
					if key_down(ctx, kc) || key_pressed(ctx, kc) {
						ctx.key_state.insert(kc, ButtonState::Released);
						events.push(Event::KeyRelease(kc));
					}
				},

			}

		}

	}

	if let Some((button, state)) = mouse_input {
		match state {
			ElementState::Pressed => {
				if mouse_up(ctx, button) || mouse_released(ctx, button) {
					ctx.mouse_state.insert(button, ButtonState::Pressed);
					events.push(Event::MousePress(button));
				}
			},
			ElementState::Released => {
				if mouse_down(ctx, button) || mouse_pressed(ctx, button) {
					ctx.mouse_state.insert(button, ButtonState::Released);
					events.push(Event::MouseRelease(button));
				}
			},
		}
	}

	if let Some(pos) = cursor_moved {
		ctx.mouse_pos = pos.into();
	}

	if let Some(delta) = mouse_delta {
		events.push(Event::MouseMove(Pos {
			x: delta.0 as i32,
			y: delta.1 as i32,
		}));
	}

	if let Some(delta) = mouse_wheel {
		events.push(Event::Scroll(delta.into()));
	}

	if let Some(size) = resized {
		ctx.width = size.width as i32;
		ctx.height = size.height as i32;
		events.push(Event::Resize(ctx.width as u32, ctx.height as u32));
	}

	if let Some(text) = text_input {
		events.push(Event::TextInput(text));
	}

	ctx.key_state
		.iter()
		.filter(|(_, &state)| state == ButtonState::Down || state == ButtonState::Pressed)
		.for_each(|(k, _)| {
			events.push(Event::KeyDown(*k));
		});

	#[cfg(all(not(target_os = "ios"), not(target_os = "android"), not(target_arch = "wasm32")))]
	while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad_ctx.next_event() {

		use gilrs::ev::EventType::*;

		match event {

			ButtonPressed(button, ..) => {
				events.push(Event::GamepadPress(id, button));
			},
			ButtonRepeated(button, ..) => {
				events.push(Event::GamepadPressRepeat(id, button));
			},
			ButtonReleased(button, ..) => {
				events.push(Event::GamepadRelease(id, button));
			},
			ButtonChanged(..) => {},
			AxisChanged(axis, val, ..) => {
				events.push(Event::GamepadAxis(id, axis, val));
			},
			Connected => {
				events.push(Event::GamepadConnect(id));
			},
			Disconnected => {
				events.push(Event::GamepadDisconnect(id));
			},
			Dropped => {},

		}

	}

	return Ok(events);

}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn str_to_mouse(s: &str) -> Option<Mouse> {
	return match s {
		"left" => Some(Mouse::Left),
		"right" => Some(Mouse::Right),
		"middle" => Some(Mouse::Middle),
		_ => None,
	}
}
