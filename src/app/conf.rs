// wengwengweng

use crate::*;
use super::*;

#[derive(Clone, Debug)]
pub struct Conf {
	pub width: i32,
	pub height: i32,
	pub title: String,
	pub hidpi: bool,
	pub resizable: bool,
	pub fullscreen: bool,
	pub always_on_top: bool,
	pub borderless: bool,
	pub transparent: bool,
	pub vsync: bool,
	pub hide_title: bool,
	pub hide_titlebar_buttons: bool,
	pub titlebar_transparent: bool,
	pub cursor_hidden: bool,
	pub cursor_locked: bool,
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

impl Conf {

	pub fn basic(title: &str, width: i32, height: i32) -> Self {
		return Self {
			title: String::from(title),
			width: width,
			height: height,
			..Default::default()
		};
	}

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
			always_on_top: false,
			borderless: false,
			transparent: false,
			vsync: true,
			hide_title: false,
			hide_titlebar_buttons: false,
			titlebar_transparent: false,
			cursor_hidden: false,
			cursor_locked: false,
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

