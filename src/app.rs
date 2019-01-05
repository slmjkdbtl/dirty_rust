// wengwengweng

//! Handles time and main loop

use std::thread;
use std::time;

use crate::*;
use crate::math::*;

// context
ctx!(APP: AppCtx);

struct AppCtx {

	dt: f32,
	time: f32,
	frame: u64,
	platform: &'static str,
	failed: bool,

}

/// init app
pub fn init() {

	ctx_init(AppCtx {

		platform: sdl2::get_platform(),
		dt: 0.0,
		time: 0.0,
		frame: 0,
		failed: false,

	});

}

/// check if app is initiated
pub fn enabled() -> bool {
	return ctx_is_ok();
}

/// start main loop, call the callback every frame
pub fn run(f: &mut FnMut()) {

	let app = ctx_get();
	let app_mut = ctx_get_mut();

	loop {

		if window::enabled() {
			window::poll_events();
		}

		if gfx::enabled() {
			gfx::reset();
		}

		f();

		if window::enabled() {
			window::swap();
		}

		app_mut.dt = 0.16;
		app_mut.frame += 1;
		app_mut.time += app.dt;
		thread::sleep(time::Duration::from_millis(16));

	}

}

/// report error and go to error screen
pub fn error(log: &str) {

	let app = ctx_get();
	let app_mut = ctx_get_mut();

	if app.failed {
		return;
	}

	app_mut.failed = true;

	if gfx::enabled() && window::enabled() {

		let (width, height) = window::size();

		loop {

			let dy = (app::time() * 0.2).sin() * 4.0;

			window::poll_events();
			gfx::reset();
			gfx::clear();

			gfx::push();
			gfx::translate(vec2!(64, 64.0 + dy));
			gfx::scale(vec2!(2.4));
			gfx::text("ERROR ♪");
			gfx::pop();

			gfx::push();
			gfx::translate(vec2!(64, 108.0 + dy));
			gfx::scale(vec2!(1.2));
			gfx::text(log);
			gfx::pop();

			gfx::line_width(3);
			gfx::color(color!(1, 1, 0, 1));
			gfx::line(rand_vec2() * vec2!(width, height), rand_vec2() * vec2!(width, height));

			if window::key_pressed(Key::Escape) {
				app::bad_quit();
			}

			window::swap();

			app_mut.dt = 0.16;
			app_mut.frame += 1;
			app_mut.time += app.dt;
			thread::sleep(time::Duration::from_millis(16));

		}

	} else {

		eprintln!("{}", log);
		app::bad_quit();

	}

}

/// get delta time between frames
pub fn dt() -> f32 {
	return ctx_get().dt;
}

/// get total number of frames passed
pub fn frame() -> u64 {
	return ctx_get().frame;
}

/// get actual time since running
pub fn time() -> f32 {
	return ctx_get().time;
}

/// quit with success code
pub fn quit() {
	std::process::exit(0);
}

pub(crate) fn bad_quit() {
	std::process::exit(1);
}

/// check if current platform is MacOS
pub fn is_macos() -> bool {
	return ctx_get().platform == "Mac OS X";
}

/// check if current platform is Windows
pub fn is_windows() -> bool {
	return ctx_get().platform == "Windows";
}

/// check if current platform is Linux
pub fn is_linux() -> bool {
	return ctx_get().platform == "Linux";
}

/// check if current platform is Android
pub fn is_android() -> bool {
	return ctx_get().platform == "Android";
}

/// check if current platform is iOS
pub fn is_ios() -> bool {
	return ctx_get().platform == "iOS";
}

