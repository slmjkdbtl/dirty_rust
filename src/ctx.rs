// wengwengweng

use crate::*;

/// Application Context
pub struct Ctx<'a> {
	pub window: &'a mut window::Window,
	pub gfx: &'a mut gfx::Gfx,
	pub app: &'a mut app::App,
	pub audio: &'a mut audio::Audio,
}

