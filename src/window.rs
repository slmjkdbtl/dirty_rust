// wengwengweng

//! Window Operations

use clipboard::ClipboardProvider;
use glutin::dpi::*;

pub use glutin::MouseCursor as CursorIcon;

use crate::*;
use math::*;

impl Ctx {

	pub fn set_fullscreen(&mut self, b: bool) {

		let window = self.windowed_ctx.window();

		if b {
			window.set_fullscreen(Some(window.get_current_monitor()));
		} else {
			window.set_fullscreen(None);
		}

	}

	pub fn is_fullscreen(&self) -> bool {
		return self.windowed_ctx.window().get_fullscreen().is_some();
	}

	pub fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	pub fn set_cursor_hidden(&mut self, b: bool) {
		self.windowed_ctx.window().hide_cursor(b);
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
			.grab_cursor(b)
			.map_err(|_| format!("failed to lock mouse cursor"))?;

		self.cursor_locked = b;

		return Ok(());

	}

	pub fn is_cursor_locked(&self) -> bool {
		return self.cursor_locked;
	}

	pub fn toggle_cursor_locked(&mut self) -> Result<()> {
		return self.set_cursor_locked(!self.is_cursor_locked());
	}

	pub fn set_cursor(&self, c: CursorIcon) {
		self.windowed_ctx.window().set_cursor(c);
	}

	pub fn set_title(&mut self, t: &str) {

		self.title = t.to_owned();

		self.windowed_ctx.window().set_title(t);

	}

	pub fn title(&self) -> &str {
		return &self.title;
	}

	pub fn dpi(&self) -> f32 {
		return self.windowed_ctx.window().get_hidpi_factor() as f32;
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
		let g_mpos: LogicalPosition = mpos.into();

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

	pub(crate) fn swap_buffers(&self) -> Result<()> {
		self.windowed_ctx
			.swap_buffers()
			.map_err(|_| format!("failed to swap buffer"))?;
		return Ok(());
	}

}

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

impl From<Vec2> for LogicalPosition {
	fn from(pos: Vec2) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

impl From<LogicalPosition> for Vec2 {
	fn from(pos: LogicalPosition) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

impl From<PhysicalPosition> for Vec2 {
	fn from(pos: PhysicalPosition) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}

