// wengwengweng

//! Input Manager

use std::collections::HashMap;

use gilrs::Gilrs;
pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;
use gctx::ctx;

use crate::*;
use crate::math::*;

// context
ctx!(INPUT: InputCtx);

struct InputCtx {

	events: sdl2::EventPump,
	key_states: HashMap<Key, ButtonState>,
	mouse_states: HashMap<Mouse, ButtonState>,
	mouse_pos: MousePos,
	mouse_delta: MouseDelta,
	scroll_delta: ScrollDelta,

}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

pub(super) fn init(e: sdl2::EventPump) {
	ctx_init(InputCtx {
		events: e,
		key_states: HashMap::new(),
		mouse_states: HashMap::new(),
		mouse_pos: MousePos::new(0, 0),
		mouse_delta: MouseDelta::new(0, 0),
		scroll_delta: ScrollDelta::new(0, 0),
	});
}

#[derive(Clone, Copy, Debug)]
pub struct MousePos {
	x: i32,
	y: i32,
}

impl MousePos {
	fn new(x: i32, y: i32) -> Self {
		return Self {
			x: x,
			y: x,
		};
	}
}

impl From<MousePos> for Vec2 {
	fn from(mpos: MousePos) -> Self {
		return vec2!(mpos.x, mpos.y);
	}
}

#[derive(Clone, Copy, Debug)]
pub struct MouseDelta {
	x: i32,
	y: i32,
}

impl MouseDelta {
	fn new(x: i32, y: i32) -> Self {
		return Self {
			x: x,
			y: x,
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

#[derive(Clone, Copy, Debug)]
pub struct ScrollDelta {
	x: i32,
	y: i32,
}

impl ScrollDelta {
	fn new(x: i32, y: i32) -> Self {
		return Self {
			x: x,
			y: x,
		};
	}
	pub fn is_none(&self) -> bool {
		return self.x == 0 && self.y == 0;
	}
}

impl From<ScrollDelta> for Vec2 {
	fn from(sdis: ScrollDelta) -> Self {
		return vec2!(sdis.x, sdis.y);
	}
}

pub(super) fn poll() {

	use sdl2::event::Event;

	let input = ctx_get();
	let input_mut = ctx_mut();
	let keyboard_state = input.events.keyboard_state();
	let mouse_state = input.events.mouse_state();
	let rmouse_state = input.events.relative_mouse_state();

	input_mut.mouse_pos = MousePos::new(mouse_state.x(), mouse_state.y());
	input_mut.mouse_delta = MouseDelta::new(rmouse_state.x(), rmouse_state.y());

	for (code, state) in &mut input_mut.key_states {
		match state {
			ButtonState::Pressed => {
				*state = ButtonState::Down;
			},
			ButtonState::Released => {
				*state = ButtonState::Up;
			},
			ButtonState::Down => {
				if !keyboard_state.is_scancode_pressed(*code) {
					*state = ButtonState::Released;
				}
			},
			_ => {}
		}
	}

	for (code, state) in &mut input_mut.mouse_states {
		match state {
			ButtonState::Pressed => {
				*state = ButtonState::Down;
			},
			ButtonState::Released => {
				*state = ButtonState::Up;
			},
			ButtonState::Down => {
				if !mouse_state.is_mouse_button_pressed(*code) {
					*state = ButtonState::Released;
				}
			},
			_ => {}
		}
	}

	for code in keyboard_state.pressed_scancodes() {
		if !input.key_states.contains_key(&code) || input.key_states[&code] == ButtonState::Up {
			input_mut.key_states.insert(code, ButtonState::Pressed);
		}
	}

	for code in mouse_state.pressed_mouse_buttons() {
		if !input.mouse_states.contains_key(&code) || input.mouse_states[&code] == ButtonState::Up {
			input_mut.mouse_states.insert(code, ButtonState::Pressed);
		}
	}

	for event in input_mut.events.poll_iter() {
		match event {
			Event::Quit {..} => {
				app::quit();
			},
			Event::MouseWheel {x, y, direction, ..} => {
				input_mut.scroll_delta = ScrollDelta::new(x, y);
			},
			_ => {}
		}
	}

}

/// get how much scrolled since last frame
pub fn scroll_delta() -> ScrollDelta {
	return ctx_get().scroll_delta;
}

/// get list of pressed keys
pub fn pressed_keys() -> Vec<Key> {

	let window = ctx_get();
	let states = &window.key_states;

	return states
		.keys()
		.filter(|&k| states[k] == ButtonState::Pressed )
		.map(|k| *k)
		.collect();

}

/// get list of down keys
pub fn down_keys() -> Vec<Key> {

	let window = ctx_get();
	let states = &window.key_states;

	return states
		.keys()
		.filter(|&k| states[k] == ButtonState::Down )
		.map(|k| *k)
		.collect();

}

/// check if a key was pressed this frame
pub fn key_pressed(k: Key) -> bool {
	return check_key_state(k, ButtonState::Pressed);
}

/// check if a key is holding down
pub fn key_down(k: Key) -> bool {
	return check_key_state(k, ButtonState::Down);
}

/// check if a key was released this frame
pub fn key_released(k: Key) -> bool {
	return check_key_state(k, ButtonState::Released);
}

/// check if a mouse button was pressed this frame
pub fn mouse_pressed(b: Mouse) -> bool {
	return check_mouse_state(b, ButtonState::Pressed);
}

/// check if a mouse button is holding down
pub fn mouse_down(b: Mouse) -> bool {
	return check_mouse_state(b, ButtonState::Down);
}

/// check if a mouse button was released this frame
pub fn mouse_released(b: Mouse) -> bool {
	return check_mouse_state(b, ButtonState::Released);
}

/// get mouse position
pub fn mouse_pos() -> MousePos {
	return ctx_get().mouse_pos;
}

/// get mouse delta position
pub fn mouse_delta() -> MouseDelta {
	return ctx_get().mouse_delta;
}

fn check_key_state(code: Key, state: ButtonState) -> bool {
	if let Some(s) = ctx_get().key_states.get(&code) {
		return s == &state;
	} else {
		return false;
	}
}

fn check_mouse_state(code: Mouse, state: ButtonState) -> bool {
	if let Some(s) = ctx_get().mouse_states.get(&code) {
		return s == &state;
	} else {
		return false;
	}
}

