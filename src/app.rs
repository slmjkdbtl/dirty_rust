// wengwengweng

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::video::GLProfile;
use std::thread;
use std::time;

use crate::*;
use crate::math::*;

ctx!(APP: AppCtx);

struct AppCtx {

	sdl_ctx: sdl2::Sdl,
	window: sdl2::video::Window,
	gl_ctx: sdl2::video::GLContext,
	events: sdl2::EventPump,
	platform: &'static str,
	running: bool,

}

enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

pub fn init(title: &str, width: u32, height: u32) {

	let sdl_ctx = sdl2::init().unwrap();
	let video = sdl_ctx.video().unwrap();
	let gl_attr = video.gl_attr();

	gl_attr.set_context_profile(GLProfile::Compatibility);
	gl_attr.set_context_version(2, 1);

	let window = video.window(title, width, height)
		.opengl()
		.build()
		.unwrap();

	let gl_ctx = window.gl_create_context().unwrap();

	gl::load_with(|name| {
		video.gl_get_proc_address(name) as *const std::os::raw::c_void
	});

	gfx::init();
	#[cfg(not(target_os = "windows"))]
	audio::init();
	res::init();

	let mut events = sdl_ctx.event_pump().unwrap();

	init_ctx(AppCtx {
		sdl_ctx: sdl_ctx,
		window: window,
		gl_ctx: gl_ctx,
		events: events,
		platform: sdl2::get_platform(),
		running: false,
	});

}

pub fn run(f: &mut FnMut()) {

	let app = get_ctx_mut();

	app.running = true;

	'running: loop {

		gfx::update();
		f();
		app.window.gl_swap_window();

		for event in app.events.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running;
				},
				Event::KeyDown { repeat: false, .. } => {
					// ...
				},
				Event::KeyUp { repeat: false, .. } => {
					// ...
				},
				_ => {}
			}
		}

		if !app.running {
			break 'running;
		}

		thread::sleep(time::Duration::from_millis(16));

	}

}

pub fn size() -> Vec2 {
	let (w, h) = get_ctx().window.size();
	return vec2!(w, h);
}

fn key_to_scancode(code: &str) -> Option<Scancode> {

	return match code {

		"a" => Some(Scancode::A),
		"b" => Some(Scancode::B),
		"c" => Some(Scancode::C),
		"d" => Some(Scancode::D),
		"e" => Some(Scancode::E),
		"f" => Some(Scancode::F),
		"g" => Some(Scancode::G),
		"h" => Some(Scancode::H),
		"i" => Some(Scancode::I),
		"j" => Some(Scancode::J),
		"k" => Some(Scancode::K),
		"l" => Some(Scancode::L),
		"m" => Some(Scancode::M),
		"n" => Some(Scancode::N),
		"o" => Some(Scancode::O),
		"p" => Some(Scancode::P),
		"q" => Some(Scancode::Q),
		"r" => Some(Scancode::R),
		"s" => Some(Scancode::S),
		"t" => Some(Scancode::T),
		"u" => Some(Scancode::U),
		"v" => Some(Scancode::V),
		"w" => Some(Scancode::W),
		"x" => Some(Scancode::X),
		"y" => Some(Scancode::Y),
		"z" => Some(Scancode::Z),
		"1" => Some(Scancode::Num1),
		"2" => Some(Scancode::Num2),
		"3" => Some(Scancode::Num3),
		"4" => Some(Scancode::Num4),
		"5" => Some(Scancode::Num5),
		"6" => Some(Scancode::Num6),
		"7" => Some(Scancode::Num7),
		"8" => Some(Scancode::Num8),
		"9" => Some(Scancode::Num9),
		"0" => Some(Scancode::Num0),
		"-" => Some(Scancode::Minus),
		"=" => Some(Scancode::Equals),
		" " => Some(Scancode::Space),
		"," => Some(Scancode::Comma),
		"." => Some(Scancode::Period),
		"/" => Some(Scancode::Slash),
		"]" => Some(Scancode::LeftBracket),
		"[" => Some(Scancode::RightBracket),
		"\\" => Some(Scancode::Backslash),
		";" => Some(Scancode::Semicolon),
		"enter" => Some(Scancode::Return),
		"esc" => Some(Scancode::Escape),
		"back" => Some(Scancode::Backspace),
		"tab" => Some(Scancode::Tab),
		"quote" => Some(Scancode::Apostrophe),
		"backquote" => Some(Scancode::Grave),
		"capslock" => Some(Scancode::CapsLock),
		"f1" => Some(Scancode::F1),
		"f2" => Some(Scancode::F2),
		"f3" => Some(Scancode::F3),
		"f4" => Some(Scancode::F4),
		"f5" => Some(Scancode::F5),
		"f6" => Some(Scancode::F6),
		"f7" => Some(Scancode::F7),
		"f8" => Some(Scancode::F8),
		"f9" => Some(Scancode::F9),
		"f10" => Some(Scancode::F10),
		"f11" => Some(Scancode::F11),
		"f12" => Some(Scancode::F12),
		"printscreen" => Some(Scancode::PrintScreen),
		"scrolllock" => Some(Scancode::ScrollLock),
		"pause" => Some(Scancode::Pause),
		"insert" => Some(Scancode::Insert),
		"home" => Some(Scancode::Home),
		"pageup" => Some(Scancode::PageUp),
		"pagedown" => Some(Scancode::PageDown),
		"delete" => Some(Scancode::Delete),
		"end" => Some(Scancode::End),
		"right" => Some(Scancode::Right),
		"left" => Some(Scancode::Left),
		"down" => Some(Scancode::Down),
		"up" => Some(Scancode::Up),
		"numlock" => Some(Scancode::NumLockClear),
		"lctrl" => Some(Scancode::LCtrl),
		"lshift" => Some(Scancode::LShift),
		"lalt" => Some(Scancode::LAlt),
		"lgui" => Some(Scancode::LGui),
		"rctrl" => Some(Scancode::RCtrl),
		"rshift" => Some(Scancode::RShift),
		"ralt" => Some(Scancode::RAlt),
		"rgui" => Some(Scancode::RGui),
		_ => None,

	};

}

pub fn key_pressed(k: &str) -> bool {
	return false;
}

pub fn key_down(k: &str) -> bool {

	match key_to_scancode(k) {
		Some(k) => {
			return get_ctx().events.keyboard_state().is_scancode_pressed(k);
		},
		None => {
			return false;
		}
	}

}

pub fn key_released(k: &str) -> bool {
	return false;
}

pub fn is_macos() -> bool {
	return get_ctx().platform == "Mac OS X";
}

pub fn is_windows() -> bool {
	return get_ctx().platform == "Windows";
}

pub fn is_linux() -> bool {
	return get_ctx().platform == "Linux";
}

pub fn is_android() -> bool {
	return get_ctx().platform == "Android";
}

pub fn is_ios() -> bool {
	return get_ctx().platform == "iOS";
}

pub fn quit() {
	get_ctx_mut().running = false;
}

