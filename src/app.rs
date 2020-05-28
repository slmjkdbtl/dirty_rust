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
	data_path: Option<&'static str>,
}

impl App {
	pub(crate) fn new(conf: &conf::Conf) -> Self {
		return Self {
			start_time: Instant::now(),
			dt: Duration::from_secs_f32(0.0),
			fps_counter: FPSCounter::new(),
			last_frame_time: Instant::now(),
			data_path: conf.data_path,
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

	#[cfg(not(web))]
	pub fn save_data<D: serde::ser::Serialize>(&self, entry: &'static str, data: D) -> Result<()> {

		let path = self.data_path
			.ok_or_else(|| "no data path specified".to_string())?;
		let data_dir = dirs_next::data_dir()
			.ok_or_else(|| "failed to get data dir".to_string())?
			.join(path);

		if !fs::exists(&data_dir) {
			fs::mkdir(&data_dir)?;
		}

		let data_file = data_dir.join(&format!("{}.json", entry));
		let content = serde_json::to_string(&data)
			.map_err(|_| format!("failed to encode json"))?;

		fs::write(data_file, content)?;

		return Ok(());

	}

	#[cfg(not(web))]
	pub fn get_data<D: for<'a> serde::de::Deserialize<'a>>(&self, entry: &'static str) -> Result<D> {

		let path = self.data_path
			.ok_or_else(|| "no data path specified".to_string())?;
		let data_dir = dirs_next::data_dir()
			.ok_or_else(|| "failed to get data dir".to_string())?
			.join(path);
		let data_file = data_dir.join(&format!("{}.json", entry));
		let content = fs::read_str(data_file)?;

		return serde_json::from_str(&content)
			.map_err(|_| format!("failed to decode json"));

	}

	// TODO: web data with local storage

	#[cfg(web)]
	pub fn save_data<D: serde::ser::Serialize>(entry: &'static str, data: D) -> Result<()> {
		todo!();
	}

	#[cfg(web)]
	pub fn get_data<D: for<'a> serde::de::Deserialize<'a>>(entry: &'static str) -> Result<D> {
		todo!();
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

