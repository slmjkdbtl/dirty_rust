// wengwengweng

//! App Lifecycles, Time and Errors

use std::thread;
use std::time::{Instant, Duration};
use std::panic;

use crate::*;

// context
ctx!(APP: AppCtx);

struct AppCtx {

	dt: f32,
	time: f32,
	platform: String,
	fps_cap: u8,

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

// 		let mut location = String::from("");

// 		if let Some(loc) = info.location() {
// 			location = format!("from '{}', line {}", loc.file(), loc.line());
// 		}

		if !app::enabled() {
			return eprintln!("{}", log);
		}

		if gfx::enabled() && window::enabled() {

			let (width, height) = window::size();

			run(&mut || {

				let dy = (app::time() * 2.0).sin() * 4.0;

				gfx::clear();

				gfx::push();

					gfx::translate(vec2!(64, 64.0 + dy));

					gfx::push();
						gfx::scale(vec2!(2.4));
						gfx::text("ERROR ♪");
					gfx::pop();

					gfx::translate(vec2!(0, 48));

					gfx::push();
						gfx::scale(vec2!(1.2));
						gfx::text(&log);
// 						gfx::translate(vec2!(0, 20));
// 						gfx::text(&location);
					gfx::pop();

				gfx::pop();

				gfx::line_width(3);
				gfx::color(color!(1, 1, 0, 1));
				gfx::line(math::rand_vec2() * vec2!(width, height), math::rand_vec2() * vec2!(width, height));

				if window::key_pressed(Key::Escape) {
					std::process::exit(1);
				}

			});

		} else {
			return eprintln!("{}", log);
		}

	}));

	ctx_init(AppCtx {

		platform: sdl2::get_platform().to_owned(),
		dt: 0.0,
		time: 0.0,
		fps_cap: 60,

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

		let start_time = Instant::now();

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

		let actual_dt = start_time.elapsed();
		let actual_dt = actual_dt.as_secs() as f32 * 1000.0 + actual_dt.subsec_millis() as f32;
		let expected_dt = 1000.0 / app.fps_cap as f32;

		if expected_dt > actual_dt {
			app_mut.dt = expected_dt as f32 / 1000.0;
			thread::sleep(Duration::from_millis((expected_dt - actual_dt) as u64));
		} else {
			app_mut.dt = actual_dt as f32 / 1000.0;
		}

		app_mut.time += app.dt;

	}

}

/// set the expected fps
pub fn set_fps_cap(f: u8) {
	ctx_get_mut().fps_cap = f;
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

/// quit with success code
pub fn quit() {
	std::process::exit(0);
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

