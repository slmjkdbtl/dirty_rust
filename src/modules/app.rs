// wengwengweng

//! Lifecycles, Time and Errors

use std::thread;
use std::time::Instant;
use std::time::Duration;
use std::panic;

use crate::*;

const FPS_CAP: u8 = 60;
const EXPECTED_DT: f32 = 1000.0 / FPS_CAP as f32;

// context
ctx!(APP: AppCtx);

struct AppCtx {

	dt: f32,
	time: f32,
	debug: bool,
	started: bool,

}

/// init app
pub fn init() {

	panic::set_hook(Box::new(|info| {

		let mut log = String::from("nonono");

		if let Some(s) = info.payload().downcast_ref::<&str>() {
			log = (*s).to_owned();
		} else if let Some(s) = info.payload().downcast_ref::<String>() {
			log = s.clone();
		}

		let mut location = String::from("");

		if let Some(loc) = info.location() {
			location = format!("from '{}', line {}", loc.file(), loc.line());
		}

		eprintln!("{}", log);

		if !enabled() || !gfx::enabled() || !window::enabled() {
			return;
		}

		let (width, height) = window::size();

		run(|| {

			let dy = (time() * 2.0).sin() * 4.0;

			g2d::push();
			g2d::text_wrap(width - 240);

			g2d::translate(vec2!(64, 64.0 + dy));

			g2d::push();
			g2d::scale(vec2!(2.4));
			g2d::text("ERROR ♪");
			g2d::pop();

			g2d::translate(vec2!(0, 48));

			g2d::push();
			g2d::scale(vec2!(1.2));
			g2d::text(&format!("{}\n\n{}", log, location));
			g2d::pop();

			g2d::pop();

			g2d::line_width(3);
			g2d::color(color!(1, 1, 0, 1));
			g2d::line(vec2!(rand!(width), rand!(height)), vec2!(rand!(width), rand!(height)));

			if window::key_pressed(Key::Escape) {
				std::process::exit(1);
			}

		});

	}));

	ctx_init(AppCtx {

		dt: 0.0,
		time: 0.0,
		debug: false,
		started: false,

	});

}

/// check if app is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

/// start main loop, call the callback every frame
pub fn run<F: FnMut()>(mut f: F) {

	let app = ctx_get();
	let app_mut = ctx_get_mut();

	app_mut.started = true;

	loop {

		let start_time = Instant::now();

		if window::enabled() {
			window::begin();
		}

		f();

		if window::enabled() {
			window::end();
		}

		let actual_dt = start_time.elapsed();
		let actual_dt = actual_dt.as_secs() as f32 * 1000.0 + actual_dt.subsec_millis() as f32;

		if EXPECTED_DT > actual_dt {
			app_mut.dt = EXPECTED_DT as f32 / 1000.0;
			thread::sleep(Duration::from_millis((EXPECTED_DT - actual_dt) as u64));
		} else {
			app_mut.dt = actual_dt as f32 / 1000.0;
		}

		app_mut.time += app.dt;

	}

}

/// get delta time between frames
pub fn dt() -> f32 {
	return ctx_get().dt;
}

/// get current framerate
pub fn fps() -> u32 {
	return (1.0 / ctx_get().dt) as u32;
}

/// get actual time since running
pub fn time() -> f32 {
	return ctx_get().time;
}

/// set debug mode
pub fn set_debug(b: bool) {
	ctx_get_mut().debug = b;
}

/// get debug mode
pub fn debug() -> bool {
	return ctx_get().debug;
}

/// quit with success code
pub fn quit() {
	std::process::exit(0);
}

/// check if current platform is MacOS
pub fn is_macos() -> bool {
	return sdl2::get_platform() == "Mac OS X";
}

/// check if current platform is Windows
pub fn is_windows() -> bool {
	return sdl2::get_platform() == "Windows";
}

/// check if current platform is Linux
pub fn is_linux() -> bool {
	return sdl2::get_platform() == "Linux";
}

/// check if current platform is Android
pub fn is_android() -> bool {
	return sdl2::get_platform() == "Android";
}

/// check if current platform is iOS
pub fn is_ios() -> bool {
	return sdl2::get_platform() == "iOS";
}

