// wengwengweng

//! Lifecycles, Time and Errors

use std::thread;
use std::time::Instant;
use std::time::Duration;
use std::panic;

use crate::*;

const FPS_CAP: u8 = 60;

// context
ctx!(APP: AppCtx);

struct AppCtx {

	dt: f32,
	time: f32,

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

		if !enabled() {
			return eprintln!("{}", log);
		}

		if gfx::enabled() && window::enabled() {

			let (width, height) = window::size();

			run(&mut || {

				let dy = (time() * 2.0).sin() * 4.0;

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
				gfx::line(vec2!(rand!(width), rand!(height)), vec2!(rand!(width), rand!(height)));

				if window::key_pressed(Key::Escape) {
					std::process::exit(1);
				}

			});

		} else {
			return eprintln!("{}", log);
		}

	}));

	ctx_init(AppCtx {

		dt: 0.0,
		time: 0.0,

	});

}

/// check if app is initiated
pub fn enabled() -> bool {
	return ctx_ok();
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
			gfx::clear();
		}

		f();

		if gfx::enabled() {
			gfx::flush();
		}

		if window::enabled() {
			window::swap();
		}

		let actual_dt = start_time.elapsed();
		let actual_dt = actual_dt.as_secs() as f32 * 1000.0 + actual_dt.subsec_millis() as f32;
		let expected_dt = 1000.0 / f32::from(FPS_CAP);

		if expected_dt > actual_dt {
			app_mut.dt = expected_dt as f32 / 1000.0;
			thread::sleep(Duration::from_millis((expected_dt - actual_dt) as u64));
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

/// quit with success code
pub fn quit() {
	std::process::exit(0);
}

#[allow(missing_docs)]
#[derive(PartialEq, Clone, Copy)]
pub enum Platform {
	MacOS,
	Windows,
	Linux,
	IOS,
	Android,
	Other,
}

/// get current platform
pub fn platform() -> Platform {
	return match sdl2::get_platform() {
		"Mac OS X" => Platform::MacOS,
		"Windows" => Platform::Windows,
		"Linux" => Platform::Linux,
		"iOS" => Platform::IOS,
		"Android" => Platform::Android,
		_ => Platform::Other,
	}
}

