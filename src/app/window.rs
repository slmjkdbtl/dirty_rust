// wengwengweng

//! Window Operations

#[cfg(not(web))]
use glutin::dpi::*;

use clipboard::ClipboardProvider;

use super::*;
use crate::math::*;
use crate::*;

pub use glutin::window::CursorIcon as CursorStyle;

impl Ctx {

	pub fn set_fullscreen(&mut self, b: bool) {

		#[cfg(not(web))] {

			let window = self.windowed_ctx.window();

			if b {
				window.set_fullscreen(Some(glutin::window::Fullscreen::Borderless(window.current_monitor())));
			} else {
				window.set_fullscreen(None);
			}

		}

	}

	pub fn is_fullscreen(&self) -> bool {

		#[cfg(not(web))]
		return self.windowed_ctx.window().fullscreen().is_some();

		#[cfg(web)]
		return false;

	}

	pub fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	pub fn set_cursor_hidden(&mut self, b: bool) {
		#[cfg(not(web))]
		self.windowed_ctx.window().set_cursor_visible(!b);
		self.cursor_hidden = b;
	}

	pub fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	pub fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	pub fn set_cursor_locked(&mut self, b: bool) -> Result<()> {

		#[cfg(not(web))]
		self.windowed_ctx.window().set_cursor_grab(b);
		self.cursor_locked = b;

		return Ok(());

	}

	pub fn is_cursor_locked(&self) -> bool {
		return self.cursor_locked;
	}

	pub fn toggle_cursor_locked(&mut self) -> Result<()> {
		return self.set_cursor_locked(!self.is_cursor_locked());
	}

	pub fn set_cursor(&self, c: CursorStyle) {
		self.windowed_ctx.window().set_cursor_icon(c);
	}

	pub fn set_title(&mut self, t: &str) {

		self.title = t.to_owned();

		#[cfg(not(web))]
		self.windowed_ctx.window().set_title(t);

		#[cfg(web)]
		stdweb::web::document().set_title(t);

	}

	pub fn title(&self) -> &str {
		return &self.title;
	}

	pub fn dpi(&self) -> f32 {

		#[cfg(not(web))]
		return self.windowed_ctx.window().scale_factor() as f32;

		#[cfg(web)]
		return 1.0;

	}

	pub fn width(&self) -> i32 {
		return self.width;
	}

	pub fn height(&self) -> i32 {
		return self.height;
	}

	pub fn set_mouse_pos(&mut self, mpos: Vec2) -> Result<()> {

		let (w, h) = (self.width as f32, self.height as f32);
		let mpos = vec2!(w / 2.0 + mpos.x, h / 2.0 - mpos.y);

// 		#[cfg(not(web))]
// 		self.windowed_ctx.window().set_cursor_position(mpos.into());
		self.mouse_pos = mpos;

		return Ok(());

	}

	pub fn get_clipboard(&mut self) -> Option<String> {
		return self.clipboard_ctx.get_contents().ok();
	}

	pub fn set_clipboard(&mut self, s: &str) -> Result<()> {
		return Ok(self.clipboard_ctx.set_contents(s.to_owned())?);
	}

}

pub(super) fn swap(ctx: &app::Ctx) -> Result<()> {
	#[cfg(not(web))]
	ctx.windowed_ctx.swap_buffers()?;
	return Ok(());
}

#[cfg(not(web))]
impl From<glutin::event::MouseScrollDelta> for Vec2 {
	fn from(delta: glutin::event::MouseScrollDelta) -> Self {
		use glutin::event::MouseScrollDelta;
		match delta {
			MouseScrollDelta::PixelDelta(pos) => {
				return vec2!(pos.x, pos.y);
			},
			MouseScrollDelta::LineDelta(x, y) => {
				return vec2!(x, y);
			}
		};
	}
}

#[cfg(not(web))]
impl From<Vec2> for LogicalPosition<f64> {
	fn from(pos: Vec2) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

#[cfg(not(web))]
impl From<LogicalPosition<f64>> for Vec2 {
	fn from(pos: LogicalPosition<f64>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

#[cfg(not(web))]
impl From<PhysicalPosition<f64>> for Vec2 {
	fn from(pos: PhysicalPosition<f64>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

#[cfg(not(web))]
impl From<PhysicalPosition<i32>> for Vec2 {
	fn from(pos: PhysicalPosition<i32>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

