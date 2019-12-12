// wengwengweng

//! Input & Events

use std::path::PathBuf;
use std::collections::HashSet;

use once_cell::sync::Lazy;

pub use sdl2::keyboard::Keycode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

static INVALID_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
	return hset![
		// backspace
		'\u{7f}',
		// return
		'\r',
		'\n',
		// esc
		'\u{1b}',
		// unknown?
		'\u{8}',
		// up/down/left/right
		'\u{f700}',
		'\u{f701}',
		'\u{f702}',
		'\u{f703}',
		// f1 - f12
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

use super::*;
use crate::*;

// TODO: wait for winit's official impl
fn is_private_use_char(c: char) -> bool {
	match c {
		'\u{E000}'..='\u{F8FF}' | '\u{F0000}'..='\u{FFFFD}' | '\u{100000}'..='\u{10FFFD}' => true,
		_ => false,
	}
}

#[derive(Clone, Copy, Debug)]
pub struct KeyMod {
	pub shift: bool,
	pub ctrl: bool,
	pub alt: bool,
	pub meta: bool,
}

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
		return self.key_states.get(&key) == Some(&ButtonState::Down) || self.key_pressed(key);
	}

	pub fn mouse_down(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Down) || self.mouse_pressed(mouse);
	}

// 	pub fn gamepad_down(&self, id: GamepadID, button: GamepadButton) -> bool {
// 		if let Some(states) = self.gamepad_button_states.get(&id) {
// 			return states.get(&button) == Some(&ButtonState::Down) || self.gamepad_pressed(id, button);
// 		} else {
// 			return false;
// 		}
// 	}

	pub fn mouse_pos(&self) -> Vec2 {

		let (w, h) = (self.width, self.height);
		let (gw, gh) = (self.gwidth(), self.gheight());
		let x = self.mouse_pos.x * gw as f32 / w as f32;
		let y = self.mouse_pos.y * gh as f32 / h as f32;

		return vec2!(x, y);

	}

	fn key_pressed(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Pressed);
	}

	fn key_released(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Released);
	}

	fn key_up(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Up) || self.key_states.get(&key).is_none();
	}

	fn mouse_pressed(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Pressed);
	}

	fn mouse_released(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Released);
	}

	fn mouse_up(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Up) || self.mouse_states.get(&mouse).is_none();
	}

// 	fn gamepad_up(&self, id: GamepadID, button: GamepadButton) -> bool {
// 		if let Some(states) = self.gamepad_button_states.get(&id) {
// 			return states.get(&button) == Some(&ButtonState::Up) || states.get(&button).is_none();
// 		} else {
// 			return true;
// 		}
// 	}

// 	fn gamepad_pressed(&self, id: GamepadID, button: GamepadButton) -> bool {
// 		if let Some(states) = self.gamepad_button_states.get(&id) {
// 			return states.get(&button) == Some(&ButtonState::Pressed);
// 		} else {
// 			return false;
// 		}
// 	}

// 	fn gamepad_released(&self, id: GamepadID, button: GamepadButton) -> bool {
// 		if let Some(states) = self.gamepad_button_states.get(&id) {
// 			return states.get(&button) == Some(&ButtonState::Released);
// 		} else {
// 			return false;
// 		}
// 	}

	pub fn key_mods(&self) -> KeyMod {
		return KeyMod {
			shift: self.key_down(Key::LShift) || self.key_down(Key::RShift),
			ctrl: self.key_down(Key::LCtrl) || self.key_down(Key::RCtrl),
			alt: self.key_down(Key::LAlt) || self.key_down(Key::RAlt),
			meta: self.key_down(Key::LGui) || self.key_down(Key::RGui),
		};
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
	Scroll(Vec2),
	CharInput(char),
// 	GamepadPress(GamepadID, GamepadButton),
// 	GamepadPressRepeat(GamepadID, GamepadButton),
// 	GamepadRelease(GamepadID, GamepadButton),
// 	GamepadAxis(GamepadID, GamepadAxis, Vec2),
// 	GamepadConnect(GamepadID),
// 	GamepadDisconnect(GamepadID),
// 	Touch(TouchID, Vec2),
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
	event_loop: &mut sdl2::EventPump,
	s: &mut impl app::State,
) -> Result<()> {

	use sdl2::event::Event as SDLEvent;

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

// 	for states in ctx.gamepad_button_states.values_mut() {
// 		for state in states.values_mut() {
// 			if state == &ButtonState::Pressed {
// 				*state = ButtonState::Down;
// 			} else if state == &ButtonState::Released {
// 				*state = ButtonState::Up;
// 			}
// 		}
// 	}

	let mut close = false;

	for event in event_loop.poll_iter() {

		match event {

			SDLEvent::Quit {..} => {
				close = true;
			},

			SDLEvent::KeyDown { keycode, .. } => {

				if let Some(key) = keycode {

					s.event(&mut ctx, &Event::KeyPressRepeat(key))?;

					if ctx.key_up(key) || ctx.key_released(key) {
						ctx.key_states.insert(key, ButtonState::Pressed);
						s.event(&mut ctx, &Event::KeyPress(key))?;
					}

				}

			},

			SDLEvent::KeyUp { keycode, .. } => {

				if let Some(key) = keycode {

					if ctx.key_down(key) || ctx.key_pressed(key) {
						ctx.key_states.insert(key, ButtonState::Released);
						s.event(&mut ctx, &Event::KeyRelease(key))?;
					}

				}

			},

			SDLEvent::MouseButtonDown { mouse_btn, .. } => {

				if ctx.mouse_up(mouse_btn) || ctx.mouse_released(mouse_btn) {
					ctx.mouse_states.insert(mouse_btn, ButtonState::Pressed);
					s.event(&mut ctx, &Event::MousePress(mouse_btn))?;
				}

			},

			SDLEvent::MouseButtonUp { mouse_btn, .. } => {

				if ctx.mouse_down(mouse_btn) || ctx.mouse_pressed(mouse_btn) {
					ctx.mouse_states.insert(mouse_btn, ButtonState::Released);
					s.event(&mut ctx, &Event::MouseRelease(mouse_btn))?;
				}

			},

			SDLEvent::MouseMotion { x, y, xrel, yrel, .. } => {

				let scale = ctx.conf.scale;

				s.event(&mut ctx, &Event::MouseMove(vec2!(xrel, yrel) / scale))?;
				ctx.mouse_pos = vec2!(x, y) / scale;

			},

			SDLEvent::MouseWheel { x, y, direction, .. } => {

				let delta = match direction {
					sdl2::mouse::MouseWheelDirection::Flipped => -vec2!(x, y),
					_ => vec2!(x, y),
				};

				s.event(&mut ctx, &Event::Scroll(delta))?;

			},

			SDLEvent::TextInput { text, .. } => {
				for ch in text.chars() {
					if !INVALID_CHARS.contains(&ch) && !ch.is_control() && !is_private_use_char(ch) {
						s.event(&mut ctx, &Event::CharInput(ch))?;
					}
				}
			},

			_ => {},

		}

	}

	if close {
		ctx.quit = true;
		return Ok(());
	}

	return Ok(());

}

