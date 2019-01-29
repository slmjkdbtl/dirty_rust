// wengwengweng

//! Window & Events

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::video::Window;
use sdl2::video::FullscreenType;
use sdl2::video::SwapInterval;

use crate::*;
use crate::math::*;

// context
ctx!(WINDOW: WindowCtx);

struct WindowCtx {

	sdl_ctx: sdl2::Sdl,
	window: Window,
	#[allow(dead_code)]
	gl_ctx: sdl2::video::GLContext,
	events: sdl2::EventPump,
	key_states: HashMap<Scancode, ButtonState>,
	mouse_states: HashMap<MouseButton, ButtonState>,
	mouse_pos: Vec2,
	size: (u32, u32),
	scale: Scale,

}

/// start window with title, width, and height
pub fn init(title: &str, width: u32, height: u32) {

	if !app::enabled() {
		panic!("can't init window without app");
	}

	let sdl_ctx = sdl2::init().expect("failed to init SDL context");
	let video = sdl_ctx.video().expect("failed to init SDL video subsystem");
	let gl_attr = video.gl_attr();

	gl_attr.set_context_profile(sdl2::video::GLProfile::Compatibility);
	gl_attr.set_context_version(2, 1);

	let window = video.window(title, width, height)
		.opengl()
		.resizable()
		.build()
		.expect("failed to create window");

	let gl_ctx = window.gl_create_context().expect("failed to create OpenGL context");

	gl::load_with(|name| {
		video.gl_get_proc_address(name) as *const std::os::raw::c_void
	});

	ctx_init(WindowCtx {

		events: sdl_ctx.event_pump().expect("failed to create event pump"),
		window: window,
		gl_ctx: gl_ctx,
		sdl_ctx: sdl_ctx,
		key_states: HashMap::new(),
		mouse_states: HashMap::new(),
		mouse_pos: vec2!(),
		size: (width, height),
		scale: Scale::X1,

	});

	gfx::init();
	g3d::init();

}

/// check if window is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

pub enum Scale {
	X1,
	X2,
	X4,
	X8,
}

/// scale entire viewport
pub fn scale(s: Scale) {
	ctx_get_mut().scale = s;
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

/// set window fullscreen state
pub fn set_fullscreen(b: bool) {

	let app_mut = ctx_get_mut();

	if b {
		app_mut.window.set_fullscreen(FullscreenType::Desktop).expect("fullscreen failed");
	} else {
		app_mut.window.set_fullscreen(FullscreenType::Off).expect("fullscreen failed");
	}

}

/// get window fullscreen state
pub fn get_fullscreen() -> bool {
	return ctx_get().window.fullscreen_state() == FullscreenType::Desktop;
}

/// show cursor
pub fn show_cursor() {
	ctx_get_mut().sdl_ctx.mouse().show_cursor(true);
}

/// hide cursor
pub fn hide_cursor() {
	ctx_get_mut().sdl_ctx.mouse().show_cursor(false);
}

/// set mouse relative state
pub fn set_relative(b: bool) {
	ctx_get_mut().sdl_ctx.mouse().set_relative_mouse_mode(b);
}

/// get mouse relative state
pub fn get_relative() -> bool {
	return ctx_get().sdl_ctx.mouse().relative_mouse_mode();
}

/// get view size
pub fn size() -> (u32, u32) {

	let window = ctx_get();
	let (w, h) = window.size;

	return match window.scale {
		Scale::X1 => (w / 1, h / 1),
		Scale::X2 => (w / 2, h / 2),
		Scale::X4 => (w / 4, h / 4),
		Scale::X8 => (w / 8, h / 8),
	};

}

/// check if a key was pressed this frame
pub fn key_pressed(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Pressed);
}

/// check if a key is holding down
pub fn key_down(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Down);
}

/// check if a key was released this frame
pub fn key_released(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Released);
}

/// check if a mouse button was pressed this frame
pub fn mouse_pressed(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Pressed);
}

/// check if a mouse button is holding down
pub fn mouse_down(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Down);
}

/// check if a mouse button was released this frame
pub fn mouse_released(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Released);
}

/// get mouse position
pub fn mouse_pos() -> Vec2 {
	return ctx_get().mouse_pos;
}

pub(crate) fn poll_events() {

	let window = ctx_get();
	let window_mut = ctx_get_mut();
	let keyboard_state = window.events.keyboard_state();
	let mouse_state = window.events.mouse_state();

	window_mut.mouse_pos = vec2!(mouse_state.x(), mouse_state.y());

	for (code, state) in &mut window_mut.key_states {
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

	for (code, state) in &mut window_mut.mouse_states {
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
		if !window.key_states.contains_key(&code) || window.key_states[&code] == ButtonState::Up {
			window_mut.key_states.insert(code, ButtonState::Pressed);
		}
	}

	for code in mouse_state.pressed_mouse_buttons() {
		if !window.mouse_states.contains_key(&code) || window.mouse_states[&code] == ButtonState::Up {
			window_mut.mouse_states.insert(code, ButtonState::Pressed);
		}
	}

	for event in window_mut.events.poll_iter() {
		match event {
			Event::Quit {..} => {
				app::quit();
			},
			_ => {}
		}
	}

}

pub(crate) fn begin() {

	match ctx_get().scale {
		Scale::X1 => {},
		Scale::X2 => gfx::scale(vec2!(2)),
		Scale::X4 => gfx::scale(vec2!(4)),
		Scale::X8 => gfx::scale(vec2!(8)),
	};

	poll_events();
	gfx::begin();

}

pub(crate) fn end() {
	gfx::end();
	swap();
}

pub(crate) fn swap() {
	ctx_get().window.gl_swap_window();
}

fn check_key_state(code: Scancode, state: ButtonState) -> bool {

	match ctx_get().key_states.get(&code) {
		Some(s) => {
			return *s == state;
		}
		None => {
			return false;
		}
	}

}

fn check_mouse_state(code: MouseButton, state: ButtonState) -> bool {

	match ctx_get().mouse_states.get(&code) {
		Some(s) => {
			return *s == state;
		}
		None => {
			return false;
		}
	}

}

