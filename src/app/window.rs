// wengwengweng

//! Window & Graphics

#[cfg(not(target_arch = "wasm32"))]
use glutin::dpi::*;
use derive_more::*;

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
	fn dpi(&self) -> f64;
	fn width(&self) -> i32;
	fn height(&self) -> i32;
	fn mouse_pos(&self) -> Pos;

}

impl Window for Ctx {

	fn set_fullscreen(&mut self, b: bool) {

		#[cfg(not(target_arch = "wasm32"))] {

			let window = self.windowed_ctx.window();

			if b {
				window.set_fullscreen(Some(window.get_current_monitor()));
				self.fullscreen = true;
			} else {
				window.set_fullscreen(None);
				self.fullscreen = false;
			}

		}

	}

	fn is_fullscreen(&self) -> bool {
		#[cfg(target_arch = "wasm32")]
		return false;
		#[cfg(not(target_arch = "wasm32"))]
		return self.windowed_ctx.window().get_fullscreen().is_some();
	}

	fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	fn set_cursor_hidden(&mut self, b: bool) {
		#[cfg(not(target_arch = "wasm32"))] {
			self.windowed_ctx.window().hide_cursor(b);
			self.cursor_hidden = b;
		}
	}

	fn is_cursor_hidden(&self) -> bool {
		return self.cursor_hidden;
	}

	fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	fn set_cursor_locked(&mut self, b: bool) -> Result<()> {
		#[cfg(not(target_arch = "wasm32"))] {
			self.windowed_ctx.window().grab_cursor(b)?;
			self.cursor_locked = b;
		}
		return Ok(());
	}

	fn is_cursor_locked(&self) -> bool {
		#[cfg(target_arch = "wasm32")]
		return false;
		#[cfg(not(target_arch = "wasm32"))]
		return self.cursor_locked;
	}

	fn toggle_cursor_locked(&mut self) -> Result<()> {
		return self.set_cursor_locked(!self.is_cursor_locked());
	}

	fn set_title(&mut self, t: &str) {
		#[cfg(not(target_arch = "wasm32"))]
		self.windowed_ctx.window().set_title(t);
	}

	fn dpi(&self) -> f64 {
		#[cfg(target_arch = "wasm32")]
		return 1.0;
		#[cfg(not(target_arch = "wasm32"))]
		return self.windowed_ctx.window().get_hidpi_factor();
	}

	fn width(&self) -> i32 {
		return self.width;
	}

	fn height(&self) -> i32 {
		return self.height;
	}

	fn mouse_pos(&self) -> Pos {
		return self.mouse_pos;
	}

}

pub(super) fn swap(ctx: &app::Ctx) -> Result<()> {
	#[cfg(not(target_arch = "wasm32"))]
	ctx.windowed_ctx.swap_buffers()?;
	return Ok(());
}

#[derive(Copy, Clone, PartialEq, Debug, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, From, Into)]
pub struct Pos {
	pub x: i32,
	pub y: i32,
}

impl Pos {
	pub(super) fn new(x: i32, y: i32) -> Self {
		return Self {
			x: x,
			y: y,
		};
	}
}

impl From<Pos> for Vec2 {
	fn from(mpos: Pos) -> Self {
		return vec2!(mpos.x, mpos.y);
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl From<LogicalPosition> for Pos {
	fn from(pos: LogicalPosition) -> Self {
		let (x, y): (i32, i32) = pos.into();
		return Self {
			x: x,
			y: y,
		};
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Pos> for LogicalPosition {
	fn from(pos: Pos) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl From<glutin::MouseScrollDelta> for Pos {
	fn from(delta: glutin::MouseScrollDelta) -> Self {
		use glutin::MouseScrollDelta;
		match delta {
			MouseScrollDelta::PixelDelta(pos) => {
				let (x, y): (i32, i32) = pos.into();
				return Self {
					x: x,
					y: y,
				};
			},
			MouseScrollDelta::LineDelta(x, y) => {
				return Self {
					x: x as i32,
					y: y as i32,
				};
			}
		};
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Vec2> for LogicalPosition {
	fn from(pos: Vec2) -> Self {
		return Self {
			x: pos.x as f64,
			y: pos.y as f64,
		};
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl From<LogicalPosition> for Vec2 {
	fn from(pos: LogicalPosition) -> Self {
		return Self {
			x: pos.x as f32,
			y: pos.y as f32,
		};
	}
}
