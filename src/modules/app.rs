// wengwengweng

//! Lifecycles, Time and Errors

use std::thread;
use std::time::Instant;
use std::time::Duration;

use gctx::*;

use crate::*;
use window::Key;

pub struct App {

	dt: f32,
	time: f32,
	debug: bool,
	running: bool,
	fps_cap: u32,

}

impl App {

	pub fn run<F: FnMut()>(&mut self, mut f: F) {

		self.running = true;

		loop {

			let start_time = Instant::now();

			f();

			let actual_dt = start_time.elapsed();
			let actual_dt = actual_dt.as_millis() as f32;
			let expected_dt = 1000.0 / self.fps_cap as f32;

			if expected_dt > actual_dt {
				self.dt = expected_dt as f32 / 1000.0;
				thread::sleep(Duration::from_millis((expected_dt - actual_dt) as u64));
			} else {
				self.dt = actual_dt as f32 / 1000.0;
			}

			self.time += self.dt;

			if !self.running {
				break;
			}

		}

	}

	pub fn on_quit<F: FnMut()>(&mut self, mut f: F) {
		// ...
	}

	pub fn on_err<F: FnMut()>(&mut self, mut f: F) {
		// ...
	}

	/// set fps cap
	pub fn cap_fps(&mut self, cap: u32) {
		self.fps_cap = cap;
	}

	/// get delta time between frames
	pub fn dt(&self) -> f32 {
		return self.dt;
	}

	/// get current framerate
	pub fn fps(&self) -> u32 {
		return (1.0 / self.dt) as u32;
	}

	/// get actual time since running
	pub fn time(&self) -> f32 {
		return self.time;
	}

	/// set debug mode
	pub fn set_debug(&mut self, b: bool) {
		self.debug = b;
	}

	/// get debug mode
	pub fn is_debug(&self) -> bool {
		return self.debug;
	}

	/// quit with success code
	pub fn quit(&mut self) {
		self.running = false;
	}

	pub fn new() -> Self {

		return Self {

			dt: 0.0,
			time: 0.0,
			debug: false,
			running: false,
			fps_cap: 60,

		};

	}

}

// context
ctx!(APP: App);

pub fn init() {

	ctx_init!(APP, App::new());

	ezpanic::set(|info: ezpanic::ErrorInfo| {

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

}

pub fn enabled() -> bool {
	return ctx_ok!(APP);
}

pub fn run<F: FnMut()>(mut f: F) {

	let app = ctx_mut!(APP);

	app.run(|| {

		if window::enabled() {
			window::begin();
		}

		f();

		if window::enabled() {
			window::end();
		}
	});

}

expose!(APP, dt() -> f32);
expose!(APP, time() -> f32);
expose!(APP, fps() -> u32);
expose!(APP, is_debug() -> bool);
expose!(APP(mut), set_debug(b: bool));
expose!(APP(mut), quit());
expose!(APP(mut), cap_fps(f: u32));

