// wengwengweng

//! Window Operations

use clipboard::ClipboardProvider;
#[cfg(not(web))]
use glutin::dpi::*;

use crate::*;
use math::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorIcon {
	Default,
	Hand,
	Cross,
	Move,
	Progress,
	Wait,
	Text,
}

impl CursorIcon {
	#[cfg(not(web))]
	fn to_winit(&self) -> glutin::window::CursorIcon {
		return match self {
			CursorIcon::Default => glutin::window::CursorIcon::Default,
			CursorIcon::Hand => glutin::window::CursorIcon::Hand,
			CursorIcon::Cross => glutin::window::CursorIcon::Crosshair,
			CursorIcon::Move => glutin::window::CursorIcon::Move,
			CursorIcon::Progress => glutin::window::CursorIcon::Progress,
			CursorIcon::Wait => glutin::window::CursorIcon::Wait,
			CursorIcon::Text => glutin::window::CursorIcon::Text,
		};
	}
	#[cfg(web)]
	fn to_web(&self) -> &'static str {
		return match self {
			CursorIcon::Default => "default",
			CursorIcon::Hand => "pointer",
			CursorIcon::Cross => "crosshair",
			CursorIcon::Move => "move",
			CursorIcon::Progress => "progress",
			CursorIcon::Wait => "wait",
			CursorIcon::Text => "text",
		};
	}
}

impl Ctx {

	pub fn set_fullscreen(&self, b: bool) {

		#[cfg(not(web))] {

			let window = self.windowed_ctx.window();

			if b {
				window.set_fullscreen(Some(glutin::window::Fullscreen::Borderless(window.current_monitor())));
			} else {
				window.set_fullscreen(None);
			}

		}

		#[cfg(web)] {
			if b {
				self.canvas.request_fullscreen();
			} else {
				self.document.exit_fullscreen();
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

		#[cfg(web)] {
			if b {
// 				self.canvas.set_attribute("style", "cursor: none");
			} else {
				// TODO: set to prev icon
// 				self.canvas.set_attribute("style", "cursor: default");
			}
		}

		self.cursor_hidden = b;

	}

	pub fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	pub fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	pub fn set_cursor_locked(&mut self, b: bool) -> Result<()> {

		self.cursor_locked = b;

		#[cfg(not(web))]
		self.windowed_ctx
			.window()
			.set_cursor_grab(b)
			.map_err(|_| format!("failed to lock mouse cursor"))?;

		#[cfg(web)] {
// 			if b {
// 				self.canvas.request_pointer_lock();
// 			} else {
// 				self.document.exit_pointer_lock();
// 			}
		}

		return Ok(());

	}

	pub fn is_cursor_locked(&self) -> bool {
		return self.cursor_locked;
	}

	pub fn toggle_cursor_locked(&mut self) -> Result<()> {
		return self.set_cursor_locked(!self.is_cursor_locked());
	}

	pub fn minimize(&self) {
		#[cfg(not(web))]
		self.windowed_ctx.window().set_minimized(true);
	}

	pub fn set_cursor(&self, c: CursorIcon) {

		#[cfg(not(web))]
		self.windowed_ctx.window().set_cursor_icon(c.to_winit());

// 		#[cfg(web)]
		// TODO: cache cursor icon
// 		self.canvas.set_attribute("style", &format!("cursor: {}", c.to_web()));

	}

	pub fn set_title(&mut self, t: &str) {

		self.title = t.to_owned();

		#[cfg(not(web))]
		self.windowed_ctx.window().set_title(t);

		#[cfg(web)] {
			self.document.set_title(t);
			self.canvas.set_attribute("alt", t);
		}

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

		#[cfg(not(web))] {

			let g_mpos: LogicalPosition<f64> = mpos.into();

			self.windowed_ctx
				.window()
				.set_cursor_position(g_mpos)
				.map_err(|_| format!("failed to set mouse position"))?
				;

		}

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

		#[cfg(not(web))]
		self.windowed_ctx
			.swap_buffers()
			.map_err(|_| format!("failed to swap buffer"))?;

		return Ok(());

	}

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

