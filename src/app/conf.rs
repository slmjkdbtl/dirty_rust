// wengwengweng

use crate::*;
use super::*;

#[derive(Clone, Debug)]
pub struct Conf {
	pub width: u32,
	pub height: u32,
	pub title: String,
	pub hidpi: bool,
	pub resizable: bool,
	pub fullscreen: bool,
	pub borderless: bool,
	pub vsync: bool,
	pub cursor_hidden: bool,
	pub cursor_relative: bool,
	pub fps_cap: Option<u16>,
	pub clear_color: Color,
	pub origin: gfx::Origin,
	pub texture_filter: gfx::FilterMode,
	pub scale_mode: gfx::ScaleMode,
	pub scale: f32,
	pub near: f32,
	pub far: f32,
	pub default_font: Option<gfx::BitmapFontData>,
}

impl Default for Conf {

	fn default() -> Self {
		return Self {
			width: 640,
			height: 480,
			title: String::new(),
			hidpi: true,
			resizable: false,
			fullscreen: false,
			borderless: false,
			vsync: true,
			cursor_hidden: false,
			cursor_relative: false,
			fps_cap: Some(60),
			clear_color: rgba!(0),
			origin: gfx::Origin::Center,
			texture_filter: gfx::FilterMode::Nearest,
			scale_mode: gfx::ScaleMode::Stretch,
			scale: 1.0,
			near: -1024.0,
			far: 1024.0,
			default_font: None,
		};
	}

}

