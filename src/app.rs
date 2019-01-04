// wengwengweng

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::video::{Window, FullscreenType, SwapInterval};
use std::thread;
use std::time;
use std::collections::HashMap;

use crate::*;
use crate::math::*;

// context
ctx!(APP: AppCtx);

struct AppCtx {

	sdl_ctx: sdl2::Sdl,
	window: Window,
	gl_ctx: sdl2::video::GLContext,
	events: sdl2::EventPump,
	platform: &'static str,
	is_running: bool,
	failed: bool,
	errlog: String,
	key_states: HashMap<Scancode, ButtonState>,
	mouse_states: HashMap<MouseButton, ButtonState>,
	dt: f32,
	time: f32,
	frame: u64,
	size: (u32, u32),

}

// public functions
pub fn init(title: &str, width: u32, height: u32) {

	let sdl_ctx = sdl2::init().unwrap();
	let video = sdl_ctx.video().unwrap();
	let gl_attr = video.gl_attr();

	gl_attr.set_context_profile(sdl2::video::GLProfile::Compatibility);
	gl_attr.set_context_version(2, 1);

	let window = video.window(title, width, height)
		.opengl()
		.resizable()
		.build()
		.unwrap();

	let gl_ctx = window.gl_create_context().unwrap();

	gl::load_with(|name| {
		video.gl_get_proc_address(name) as *const std::os::raw::c_void
	});

	video.gl_set_swap_interval(SwapInterval::VSync).expect("vsync failed");

	ctx_init(AppCtx {

		events: sdl_ctx.event_pump().unwrap(),
		window: window,
		gl_ctx: gl_ctx,
		sdl_ctx: sdl_ctx,
		platform: sdl2::get_platform(),
		key_states: HashMap::new(),
		mouse_states: HashMap::new(),
		is_running: false,
		failed: false,
		errlog: String::new(),
		dt: 0.0,
		time: 0.0,
		frame: 0,
		size: (width, height),

	});

}

pub fn enabled() -> bool {
	return ctx_is_ok();
}

pub fn run(f: &mut FnMut()) {

	let app = ctx_get();
	let app_mut = ctx_get_mut();
	let (width, height) = app::size();
	let keyboard_state = app.events.keyboard_state();
	let mouse_state = app.events.mouse_state();

	app_mut.is_running = true;

	'running: loop {

		for event in app_mut.events.poll_iter() {

			match event {

				Event::Quit {..} => {
					break 'running;
				},

				Event::KeyDown { repeat: false, .. } => {
					for code in keyboard_state.pressed_scancodes() {
						if !app.key_states.contains_key(&code) || app.key_states[&code] == ButtonState::Up {
							app_mut.key_states.insert(code, ButtonState::Pressed);
						}
					}
				},

				Event::MouseButtonDown { .. } => {
					for code in mouse_state.pressed_mouse_buttons() {
						if !app.mouse_states.contains_key(&code) || app.mouse_states[&code] == ButtonState::Up {
							app_mut.mouse_states.insert(code, ButtonState::Pressed);
						}
					}
				},

				_ => {}

			}

		}

		gfx::reset();

		if app.failed {

			let dy = (app::time() * 0.2).sin() * 4.0;

			gfx::clear();

			gfx::push();
			gfx::translate(vec2!(64, 64.0 + dy));
			gfx::scale(vec2!(2.4));
			gfx::text("OH NO ♪");
			gfx::pop();

			gfx::push();
			gfx::translate(vec2!(64, 108.0 + dy));
			gfx::scale(vec2!(1.2));
			gfx::text(&app.errlog);
			gfx::pop();

			gfx::line_width(3);
			gfx::color(color!(1, 1, 0, 1));
			gfx::line(rand_vec2() * vec2!(width, height), rand_vec2() * vec2!(width, height));

		} else {
			f();
		}

		swap();

		if !app.is_running {
			break 'running;
		}

		for (code, state) in &mut app_mut.key_states {
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

		for (code, state) in &mut app_mut.mouse_states {
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

		app_mut.dt = 0.16;
		app_mut.frame += 1;
		app_mut.time += app.dt;
		thread::sleep(time::Duration::from_millis(16));

	}

}

pub fn error(log: &str) {

	let app_mut = ctx_get_mut();

	app_mut.failed = true;
	app_mut.errlog = log.to_owned();

}

pub fn dt() -> f32 {
	return ctx_get().dt;
}

pub fn frame() -> u64 {
	return ctx_get().frame;
}

pub fn time() -> f32 {
	return ctx_get().time;
}

pub fn set_fullscreen(b: bool) {

	let app_mut = ctx_get_mut();

	if b {
		app_mut.window.set_fullscreen(FullscreenType::Desktop).expect("fullscreen failed");
	} else {
		app_mut.window.set_fullscreen(FullscreenType::Off).expect("fullscreen failed");
	}

}

pub fn get_fullscreen() -> bool {
	return ctx_get().window.fullscreen_state() == FullscreenType::Desktop;
}

pub fn show_cursor() {
	ctx_get_mut().sdl_ctx.mouse().show_cursor(true);
}

pub fn hide_cursor() {
	ctx_get_mut().sdl_ctx.mouse().show_cursor(false);
}

pub fn set_relative(b: bool) {
	ctx_get_mut().sdl_ctx.mouse().set_relative_mouse_mode(b);
}

pub fn get_relative() -> bool {
	return ctx_get().sdl_ctx.mouse().relative_mouse_mode();
}

pub fn quit() {
	ctx_get_mut().is_running = false;
}

pub fn size() -> (u32, u32) {
	return ctx_get().size;
}

pub fn key_pressed(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Pressed);
}

pub fn key_down(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Down);
}

pub fn key_released(k: Scancode) -> bool {
	return check_key_state(k, ButtonState::Released);
}

pub fn mouse_pressed(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Pressed);
}

pub fn mouse_down(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Down);
}

pub fn mouse_released(b: MouseButton) -> bool {
	return check_mouse_state(b, ButtonState::Released);
}

pub fn is_macos() -> bool {
	return ctx_get().platform == "Mac OS X";
}

pub fn is_windows() -> bool {
	return ctx_get().platform == "Windows";
}

pub fn is_linux() -> bool {
	return ctx_get().platform == "Linux";
}

pub fn is_android() -> bool {
	return ctx_get().platform == "Android";
}

pub fn is_ios() -> bool {
	return ctx_get().platform == "iOS";
}

// private structs
#[derive(Debug, PartialEq)]
enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

// private functions
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

