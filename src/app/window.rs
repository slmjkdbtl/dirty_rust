// wengwengweng

//! Window Operations

#[cfg(not(web))]
use glutin::dpi::*;

use super::*;
use crate::math::*;
use crate::*;

pub trait Window {

	fn set_fullscreen(&mut self, b: bool);
	fn is_fullscreen(&self) -> bool;
	fn toggle_fullscreen(&mut self);

	fn set_cursor_hidden(&mut self, b: bool);
	fn is_cursor_hidden(&self) -> bool;
	fn toggle_cursor_hidden(&mut self);

	fn set_cursor_locked(&mut self, b: bool) -> Result<()>;
	fn is_cursor_locked(&self) -> bool;
	fn toggle_cursor_locked(&mut self) -> Result<()>;

	fn set_title(&mut self, t: &str);
	fn title(&self) -> &str;

	fn dpi(&self) -> f32;

	fn width(&self) -> i32;
	fn height(&self) -> i32;
	fn gwidth(&self) -> i32;
	fn gheight(&self) -> i32;

	fn set_mouse_pos(&mut self, p: Vec2) -> Result<()>;

}

impl Window for Ctx {

	#[cfg(not(web))]
	fn set_fullscreen(&mut self, b: bool) {

		let window = self.windowed_ctx.window();

		if b {
			window.set_fullscreen(Some(window.get_current_monitor()));
		} else {
			window.set_fullscreen(None);
		}

	}

	#[cfg(web)]
	fn set_fullscreen(&mut self, b: bool) {
		// ...
	}

	#[cfg(not(web))]
	fn is_fullscreen(&self) -> bool {
		return self.windowed_ctx.window().get_fullscreen().is_some();
	}

	#[cfg(web)]
	fn is_fullscreen(&self) -> bool {
		return false;
	}

	fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	#[cfg(not(web))]
	fn set_cursor_hidden(&mut self, b: bool) {
		self.windowed_ctx.window().hide_cursor(b);
		self.cursor_hidden = b;
	}

	#[cfg(web)]
	fn set_cursor_hidden(&mut self, b: bool) {
		// ...
	}

	fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	#[cfg(not(web))]
	fn set_cursor_locked(&mut self, b: bool) -> Result<()> {
		self.windowed_ctx.window().grab_cursor(b)?;
		self.cursor_locked = b;
		return Ok(());
	}

	#[cfg(web)]
	fn set_cursor_locked(&mut self, b: bool) -> Result<()> {
		return Ok(());
	}

	fn is_cursor_locked(&self) -> bool {
		return self.cursor_locked;
	}

	fn toggle_cursor_locked(&mut self) -> Result<()> {
		return self.set_cursor_locked(!self.is_cursor_locked());
	}

	fn set_title(&mut self, t: &str) {

		self.title = t.to_owned();

		#[cfg(not(web))]
		self.windowed_ctx.window().set_title(t);

		#[cfg(web)]
		stdweb::web::document().set_title(t);

	}

	fn title(&self) -> &str {
		return &self.title;
	}

	#[cfg(not(web))]
	fn dpi(&self) -> f32 {
		return self.windowed_ctx.window().get_hidpi_factor() as f32;
	}

	#[cfg(web)]
	fn dpi(&self) -> f32 {
		return 1.0;
	}

	fn width(&self) -> i32 {
		return self.width;
	}

	fn height(&self) -> i32 {
		return self.height;
	}

	fn gwidth(&self) -> i32 {
		return self.conf.width;
	}

	fn gheight(&self) -> i32 {
		return self.conf.height;
	}

	fn set_mouse_pos(&mut self, p: Vec2) -> Result<()> {

		let offset = self.conf.origin.as_pt() / 2.0 + vec2!(0.5) * vec2!(self.width(), self.height());
		let mpos = p + offset;

		#[cfg(not(web))]
		self.windowed_ctx.window().set_cursor_position(mpos.into())?;
		self.mouse_pos = mpos;

		return Ok(());

	}

}

pub(super) fn swap(ctx: &app::Ctx) -> Result<()> {
	#[cfg(not(web))]
	ctx.windowed_ctx.swap_buffers()?;
	return Ok(());
}

#[cfg(not(web))]
impl From<glutin::MouseScrollDelta> for Vec2 {
	fn from(delta: glutin::MouseScrollDelta) -> Self {
		use glutin::MouseScrollDelta;
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
impl From<Vec2> for LogicalPosition {
	fn from(pos: Vec2) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

#[cfg(not(web))]
impl From<LogicalPosition> for Vec2 {
	fn from(pos: LogicalPosition) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

