// wengwengweng

//! Application Lifecycle

use std::time::Duration;
use instant::Instant;

use crate::*;

/// Provides Information to the Application Lifecycle
pub struct App {
	last_frame_time: Instant,
	fps_counter: FPSCounter,
	start_time: Instant,
	dt: Duration,
}

impl App {
	pub(crate) fn new(conf: &conf::Conf) -> Self {
		return Self {
			start_time: Instant::now(),
			dt: Duration::from_secs_f32(0.0),
			fps_counter: FPSCounter::new(),
			last_frame_time: Instant::now(),
		};
	}
}

impl App {

	pub(crate) fn tick(&mut self) {
		self.dt = self.last_frame_time.elapsed();
		self.fps_counter.tick(self.dt);
		self.last_frame_time = Instant::now();
	}

	/// current run time
	pub fn time(&self) -> Duration {
		return self.start_time.elapsed();
	}

	/// time since last frame
	pub fn dt(&self) -> Duration {
		return self.dt;
	}

	/// current fps stat (frames per second)
	pub fn fps(&self) -> u16 {
		return self.fps_counter.fps();
	}

}

struct FPSCounter {
	frames: usize,
	timer: Duration,
	fps: u16,
}

impl FPSCounter {

	pub fn new() -> Self {
		return Self {
			frames: 0,
			timer: Duration::from_secs(0),
			fps: 0,
		}
	}

	pub fn tick(&mut self, dt: Duration) {

		self.frames += 1;
		self.timer += dt;

		if self.timer.as_secs_f32() >= 1.0 {
			self.fps = self.frames as u16;
			self.timer = Duration::from_secs(0);
			self.frames = 0;
		}

	}

	pub fn fps(&self) -> u16 {
		return self.fps;
	}

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Platform {
	MacOS,
	Windows,
	Linux,
	WASM,
	IOS,
	Android,
}

#[cfg(macos)]
pub const PLATFORM: Platform = Platform::MacOS;
#[cfg(windows)]
pub const PLATFORM: Platform = Platform::Windows;
#[cfg(linux)]
pub const PLATFORM: Platform = Platform::Linux;
#[cfg(ios)]
pub const PLATFORM: Platform = Platform::IOS;
#[cfg(android)]
pub const PLATFORM: Platform = Platform::Android;
#[cfg(web)]
pub const PLATFORM: Platform = Platform::WASM;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Env {
	Web,
	Desktop,
	Mobile,
}

#[cfg(web)]
pub const ENV: Env = Env::Web;
#[cfg(desktop)]
pub const ENV: Env = Env::Desktop;
#[cfg(mobile)]
pub const ENV: Env = Env::Mobile;

