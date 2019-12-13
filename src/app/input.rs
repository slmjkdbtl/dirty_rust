// wengwengweng

//! Input & Events

use std::path::PathBuf;
use std::collections::HashSet;

use once_cell::sync::Lazy;

pub use sdl2::keyboard::Keycode as Key;
pub use sdl2::mouse::MouseButton as Mouse;
pub use sdl2::controller::Button as GamepadButton;

const JOYSTICK_RANGE: f32 = 32767.0;

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

pub type GamepadID = i32;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum GamepadAxis {
	Left,
	Right,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum GamepadTrigger {
	Left,
	Right,
}

pub(super) struct Gamepad {
	sdl_gamepad: sdl2::controller::GameController,
	buttons: HashMap<GamepadButton, ButtonState>,
	axis: HashMap<GamepadAxis, Vec2>,
	triggers: HashMap<GamepadTrigger, f32>,
}

impl Gamepad {
	pub fn new(g: sdl2::controller::GameController) -> Self {
		return Self {
			sdl_gamepad: g,
			buttons: hmap![],
			axis: hmap![],
			triggers: hmap![],
		};
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

	pub fn gamepad_down(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(gamepad) = self.gamepads.get(&id) {
			return gamepad.buttons.get(&button) == Some(&ButtonState::Down) || self.gamepad_pressed(id, button);
		} else {
			return false;
		}
	}

	pub fn mouse_pos(&self) -> Vec2 {
		return self.mouse_pos;
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

	fn gamepad_up(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(gamepad) = self.gamepads.get(&id) {
			return gamepad.buttons.get(&button) == Some(&ButtonState::Up) || gamepad.buttons.get(&button).is_none();
		} else {
			return false;
		}
	}

	fn gamepad_pressed(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(gamepad) = self.gamepads.get(&id) {
			return gamepad.buttons.get(&button) == Some(&ButtonState::Pressed);
		} else {
			return false;
		}
	}

	fn gamepad_released(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(gamepad) = self.gamepads.get(&id) {
			return gamepad.buttons.get(&button) == Some(&ButtonState::Released);
		} else {
			return false;
		}
	}

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
	GamepadPress(GamepadID, GamepadButton),
	GamepadRelease(GamepadID, GamepadButton),
	GamepadAxis(GamepadID, GamepadAxis, Vec2),
	GamepadTrigger(GamepadID, GamepadTrigger, f32),
	GamepadConnect(GamepadID),
	GamepadDisconnect(GamepadID),
// 	Touch(TouchID, Vec2),
	Resize(i32, i32),
	FileDrop(PathBuf),
	Focus(bool),
	Hidden(bool),
	MouseOver(bool),
	ClipboardUpdate,
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
	use sdl2::event::WindowEvent as SDLWinEvent;

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

	for gamepad in &mut ctx.gamepads.values_mut() {
		for state in gamepad.buttons.values_mut() {
			if state == &ButtonState::Pressed {
				*state = ButtonState::Down;
			} else if state == &ButtonState::Released {
				*state = ButtonState::Up;
			}
		}
	}

	let mut close = false;

	for event in event_loop.poll_iter() {

		match event {

			SDLEvent::Window { win_event, .. } => {
				match win_event {
					SDLWinEvent::FocusGained => {
						s.event(&mut ctx, &Event::Focus(true))?;
					},
					SDLWinEvent::FocusLost => {
						s.event(&mut ctx, &Event::Focus(false))?;
					},
					SDLWinEvent::Enter => {
						s.event(&mut ctx, &Event::MouseOver(true))?;
					},
					SDLWinEvent::Leave => {
						s.event(&mut ctx, &Event::MouseOver(false))?;
					},
					SDLWinEvent::Resized(w, h) => {
						s.event(&mut ctx, &Event::Resize(w, h))?;
					},
					SDLWinEvent::Hidden => {
						s.event(&mut ctx, &Event::Hidden(true))?;
					},
					SDLWinEvent::Shown => {
						s.event(&mut ctx, &Event::Hidden(false))?;
					},
					_ => {},
				}
			},

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

				let (w, h) = (ctx.width() as f32, ctx.height() as f32);
				let (gw, gh) = (ctx.gwidth() as f32, ctx.gheight() as f32);
				let offset = (ctx.conf.origin.as_pt() / 2.0 + vec2!(0.5)) * vec2!(w, h);
				let (pos, aw, ah) = ctx.cur_viewport();
				let mpos = (vec2!(x, y) - offset - pos) * vec2!(gw / aw, gh / ah);

				ctx.mouse_pos = mpos;

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
					if
						!INVALID_CHARS.contains(&ch)
						&& !ch.is_control()
						&& !is_private_use_char(ch)
					{
						s.event(&mut ctx, &Event::CharInput(ch))?;
					}
				}
			},

			// TODO: repeat
			SDLEvent::ControllerButtonDown { which, button, .. } => {

				if ctx.gamepad_up(which, button) || ctx.gamepad_released(which, button) {
					if let Some(gamepad) = ctx.gamepads.get_mut(&which) {
						gamepad.buttons
							.insert(button, ButtonState::Pressed);
						s.event(&mut ctx, &Event::GamepadPress(which, button))?;
					}
				}

			},

			SDLEvent::ControllerButtonUp { which, button, .. } => {

				if ctx.gamepad_down(which, button) || ctx.gamepad_pressed(which, button) {
					if let Some(gamepad) = ctx.gamepads.get_mut(&which) {
						gamepad.buttons
							.insert(button, ButtonState::Released);
						s.event(&mut ctx, &Event::GamepadRelease(which, button))?;
					}
				}

			},

			SDLEvent::ControllerAxisMotion { which, axis, value, .. } => {

				use sdl2::controller::Axis;

				let value = f32::round((value as f32 / JOYSTICK_RANGE) * 10.0) / 10.0;

				if let Some(gamepad) = ctx.gamepads.get_mut(&which) {

					match axis {

						Axis::LeftX => {
							let val = gamepad.axis.entry(GamepadAxis::Left).or_insert(vec2!(0));
							if val.x != value {
								let nv = vec2!(value, val.y);
								*val = nv;
								s.event(&mut ctx, &Event::GamepadAxis(which, GamepadAxis::Left, nv))?;
							}
						},

						Axis::LeftY => {
							let val = gamepad.axis.entry(GamepadAxis::Left).or_insert(vec2!(0));
							if val.x != value {
								let nv = vec2!(val.x, value);
								*val = nv;
								s.event(&mut ctx, &Event::GamepadAxis(which, GamepadAxis::Left, nv))?;
							}
						},

						Axis::RightX => {
							let val = gamepad.axis.entry(GamepadAxis::Right).or_insert(vec2!(0));
							if val.x != value {
								let nv = vec2!(value, val.y);
								*val = nv;
								s.event(&mut ctx, &Event::GamepadAxis(which, GamepadAxis::Right, nv))?;
							}
						},
						Axis::RightY => {
							let val = gamepad.axis.entry(GamepadAxis::Right).or_insert(vec2!(0));
							if val.x != value {
								let nv = vec2!(val.x, value);
								*val = nv;
								s.event(&mut ctx, &Event::GamepadAxis(which, GamepadAxis::Right, nv))?;
							}
						},
						Axis::TriggerLeft => {
							let val = gamepad.triggers.entry(GamepadTrigger::Left).or_insert(0.0);
							if val != &value {
								*val = value;
								s.event(&mut ctx, &Event::GamepadTrigger(which, GamepadTrigger::Left, value))?;
							}
						},
						Axis::TriggerRight => {
							let val = gamepad.triggers.entry(GamepadTrigger::Right).or_insert(0.0);
							if val != &value {
								*val = value;
								s.event(&mut ctx, &Event::GamepadTrigger(which, GamepadTrigger::Right, value))?;
							}
						},
					}
				}

			},

			SDLEvent::ControllerDeviceAdded { which, .. } => {

				if let Ok(c) = ctx.gamepad_sys.open(which as u32) {
					ctx.gamepads.insert(c.instance_id(), Gamepad::new(c));
				}

				s.event(&mut ctx, &Event::GamepadConnect(which))?;

			}

			SDLEvent::ControllerDeviceRemoved { which, .. } => {
				ctx.gamepads.remove(&which);
				s.event(&mut ctx, &Event::GamepadDisconnect(which))?;
			},

			SDLEvent::ClipboardUpdate { .. } => {
				s.event(&mut ctx, &Event::ClipboardUpdate)?;
			},

			SDLEvent::DropFile { filename, .. } => {
				s.event(&mut ctx, &Event::FileDrop(PathBuf::from(filename)))?;
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

