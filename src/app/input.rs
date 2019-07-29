// wengwengweng

use std::collections::HashSet;

use glutin::ElementState;
pub use glutin::ModifiersState as Mod;
pub use glutin::VirtualKeyCode as Key;
pub use glutin::MouseButton as Mouse;

use super::*;
use crate::*;
use window::Pos;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

pub trait Input {

	fn down_keys(&self) -> HashSet<Key>;
	fn key_down(&self, key: Key) -> bool;
	fn key_pressed(&self, key: Key) -> bool;
	fn key_released(&self, key: Key) -> bool;
	fn key_up(&self, key: Key) -> bool;
	fn key_pressed_repeat(&self, key: Key) -> bool;
	fn mouse_down(&self, mouse: Mouse) -> bool;
	fn mouse_pressed(&self, mouse: Mouse) -> bool;
	fn mouse_released(&self, mouse: Mouse) -> bool;
	fn mouse_up(&self, mouse: Mouse) -> bool;
	fn mouse_pos(&self) -> Pos;
	fn mouse_delta(&self) -> Option<Pos>;
	fn scroll_delta(&self) -> Option<Pos>;
	fn text_input(&self) -> Option<String>;

}

impl Input for app::Ctx {

	fn down_keys(&self) -> HashSet<Key> {

		use ButtonState::*;

		return self.key_state
			.iter()
			.filter(|(_, &state)| state == Down || state == Pressed)
			.map(|(key, _)| *key)
			.collect();

	}

	fn key_down(&self, key: Key) -> bool {
		return self.key_state.get(&key) == Some(&ButtonState::Down) || self.key_pressed(key);
	}

	fn key_pressed(&self, key: Key) -> bool {
		return self.key_state.get(&key) == Some(&ButtonState::Pressed);
	}

	fn key_released(&self, key: Key) -> bool {
		return self.key_state.get(&key) == Some(&ButtonState::Released);
	}

	fn key_up(&self, key: Key) -> bool {
		return self.key_state.get(&key) == Some(&ButtonState::Up) || self.key_state.get(&key).is_none();
	}

	fn key_pressed_repeat(&self, _key: Key) -> bool {
		unimplemented!();
	}

	fn mouse_down(&self, mouse: Mouse) -> bool {
		return self.mouse_state.get(&mouse) == Some(&ButtonState::Down) || self.mouse_pressed(mouse);
	}

	fn mouse_pressed(&self, mouse: Mouse) -> bool {
		return self.mouse_state.get(&mouse) == Some(&ButtonState::Pressed);
	}

	fn mouse_released(&self, mouse: Mouse) -> bool {
		return self.mouse_state.get(&mouse) == Some(&ButtonState::Released);
	}

	fn mouse_up(&self, mouse: Mouse) -> bool {
		return self.mouse_state.get(&mouse) == Some(&ButtonState::Up) || self.mouse_state.get(&mouse).is_none();
	}

	fn mouse_pos(&self) -> Pos {
		return self.mouse_pos;
	}

	fn mouse_delta(&self) -> Option<Pos> {
		return self.mouse_delta;
	}

	fn scroll_delta(&self) -> Option<Pos> {
		return self.scroll_delta;
	}

	fn text_input(&self) -> Option<String> {
		return self.text_input.clone();
	}

}

pub(super) fn poll(ctx: &mut app::Ctx) -> Result<()> {

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

	ctx.mouse_delta = None;
	ctx.scroll_delta = None;
	ctx.text_input = None;

	let mut keyboard_input = None;
	let mut mouse_input = None;
	let mut cursor_moved = None;
	let mut mouse_wheel = None;
	let mut text_input = None;
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

				CloseRequested => close = true,

				_ => {},

			},

			_ => {},

		};

	});

	if close {
		ctx.quit = true;
		return Ok(());
	}

	if let Some(input) = keyboard_input {
		if let Some(kc) = input.virtual_keycode {
			match input.state {
				ElementState::Pressed => {
					if ctx.key_up(kc) || ctx.key_released(kc) {
						ctx.key_state.insert(kc, ButtonState::Pressed);
					}
				},
				ElementState::Released => {
					if ctx.key_down(kc) || ctx.key_pressed(kc) {
						ctx.key_state.insert(kc, ButtonState::Released);
					}
				},
			}
		}
	}

	if let Some((button, state)) = mouse_input {
		match state {
			ElementState::Pressed => {
				if ctx.mouse_up(button) || ctx.mouse_released(button) {
					ctx.mouse_state.insert(button, ButtonState::Pressed);
				}
			},
			ElementState::Released => {
				if ctx.mouse_down(button) || ctx.mouse_pressed(button) {
					ctx.mouse_state.insert(button, ButtonState::Released);
				}
			},
		}
	}

	if let Some(pos) = cursor_moved {

		let pos: Pos = pos.into();

		ctx.mouse_delta = Some((pos - ctx.mouse_pos).into());
		ctx.mouse_pos = pos;

	}

	if let Some(delta) = mouse_wheel {
		ctx.scroll_delta = Some(delta.into());
	}

	ctx.text_input = text_input;

// 	while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad_ctx.next_event() {
// 	}

	return Ok(());

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

