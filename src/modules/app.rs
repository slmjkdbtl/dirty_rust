// wengwengweng

//! Lifecycles, Time and Errors

use std::thread;
use std::time::Instant;
use std::time::Duration;
use std::panic;
use std::panic::PanicInfo;

use gctx::*;

use crate::*;
use window::Key;

// context
ctx!(APP: AppCtx);

struct AppCtx {

	dt: f32,
	time: f32,
	debug: bool,
	started: bool,
	fps_cap: u32,

}

/// init app
pub fn init() {

	on_err(|info: ErrorInfo| {

		if let Some(message) = &info.message {
			eprintln!("{}", message);
		}

		if let Some(location) = &info.location {
			eprintln!("from '{}', line {}", location.file, location.line);
		}

		if !enabled() || !gfx::enabled() || !window::enabled() {
			return;
		}

		let (width, height) = window::size().into();

		run(|| {

			let dy = (time() * 2.0).sin() * 4.0;

			g2d::reset();
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

			if let Some(message) = &info.message {
				g2d::text(&format!("{}", message));
			}

			g2d::pop();

			g2d::pop();

			g2d::line_width(3);
			g2d::color(color!(1, 1, 0, 1));
			g2d::line(vec2!(rand!(width), rand!(height)), vec2!(rand!(width), rand!(height)));

			if window::key_pressed(Key::Escape) {
				std::process::exit(1);
			}

		});

	});

	ctx_init!(APP, AppCtx {

		dt: 0.0,
		time: 0.0,
		debug: false,
		started: false,
		fps_cap: 60,

	});

}

/// check if app is initiated
pub fn enabled() -> bool {
	return ctx_ok!(APP);
}

/// start main loop, call the callback every frame
pub fn run<F: FnMut()>(mut f: F) {

	let app = ctx_get!(APP);
	let app_mut = ctx_mut!(APP);

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
		let actual_dt = actual_dt.as_millis() as f32;
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

pub struct ErrorInfo {
	message: Option<String>,
	location: Option<ErrorLocation>,
}

pub struct ErrorLocation {
	file: String,
	line: u32,
}

/// custom error handler
pub fn on_err<F: 'static + Fn(ErrorInfo) + Send + Sync>(f: F) {

	panic::set_hook(Box::new(move |info: &PanicInfo| {

		let mut message = None;

		if let Some(s) = info.payload().downcast_ref::<&str>() {
			message = Some((*s).to_owned());
		} else if let Some(s) = info.payload().downcast_ref::<String>() {
			message = Some(s.clone());
		}

		let mut location = None;

		if let Some(loc) = info.location() {

			let file = loc.file();
			let line = loc.line();

			location = Some(ErrorLocation {
				file: file.to_owned(),
				line: line,
			});

		}

		f(ErrorInfo {
			message: message,
			location: location,
		});

	}));

}

/// set fps cap
pub fn cap_fps(cap: u32) {
	ctx_mut!(APP).fps_cap = cap;
}

/// get delta time between frames
pub fn dt() -> f32 {
	return ctx_get!(APP).dt;
}

/// get current framerate
pub fn fps() -> u32 {
	return (1.0 / ctx_get!(APP).dt) as u32;
}

/// get actual time since running
pub fn time() -> f32 {
	return ctx_get!(APP).time;
}

/// set debug mode
pub fn set_debug(b: bool) {
	ctx_mut!(APP).debug = b;
}

/// get debug mode
pub fn debug() -> bool {
	return ctx_get!(APP).debug;
}

/// quit with success code
pub fn quit() {
	std::process::exit(0);
}

