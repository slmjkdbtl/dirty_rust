// wengwengweng

//! Window Operations

use glutin::dpi::*;

use clipboard::ClipboardProvider;

use super::*;
use crate::math::*;
use crate::*;

pub use glutin::window::CursorIcon as CursorStyle;

impl Ctx {

	pub fn set_fullscreen(&mut self, b: bool) {

		let window = self.windowed_ctx.window();

		if b {
			window.set_fullscreen(Some(glutin::window::Fullscreen::Borderless(window.current_monitor())));
		} else {
			window.set_fullscreen(None);
		}

	}

	pub fn is_fullscreen(&self) -> bool {
		return self.windowed_ctx.window().fullscreen().is_some();
	}

	pub fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	pub fn set_cursor_hidden(&mut self, b: bool) {
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

		self.windowed_ctx
			.window()
			.set_cursor_grab(b)
			.map_err(|_| format!("failed to set mouse cursor"))?;

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

		self.windowed_ctx.window().set_title(t);

	}

	pub fn title(&self) -> &str {
		return &self.title;
	}

	pub fn dpi(&self) -> f32 {
		return self.windowed_ctx.window().scale_factor() as f32;
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
		let g_mpos: LogicalPosition<f64> = mpos.into();

		self.windowed_ctx
			.window()
			.set_cursor_position(g_mpos)
			.map_err(|_| format!("failed to set mouse position"))?
			;

		self.mouse_pos = mpos;

		return Ok(());

	}

	pub fn get_clipboard(&mut self) -> Option<String> {
		return self.clipboard_ctx.get_contents().ok();
	}

	pub fn set_clipboard(&mut self, s: &str) -> Result<()> {

		self.clipboard_ctx
			.set_contents(s.to_owned())
			.map_err(|_| format!("failed to set clipboard"))?;

		return Ok(());

	}

}

pub(super) fn swap(ctx: &app::Ctx) -> Result<()> {
	ctx.windowed_ctx
		.swap_buffers()
		.map_err(|_| format!("failed to swap buffer"))?;
	return Ok(());
}

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

impl From<Vec2> for LogicalPosition<f64> {
	fn from(pos: Vec2) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

impl From<LogicalPosition<f64>> for Vec2 {
	fn from(pos: LogicalPosition<f64>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

impl From<PhysicalPosition<f64>> for Vec2 {
	fn from(pos: PhysicalPosition<f64>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

impl From<PhysicalPosition<i32>> for Vec2 {
	fn from(pos: PhysicalPosition<i32>) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

