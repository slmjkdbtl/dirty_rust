// wengwengweng

//! Window Operations

use super::*;
use crate::*;

pub use sdl2::mouse::SystemCursor as CursorStyle;

impl Ctx {

	pub fn set_fullscreen(&mut self, b: bool) -> Result<()> {

		#[cfg(not(web))] {

			use sdl2::video::FullscreenType;

			self.window.set_fullscreen(if b {
				FullscreenType::Desktop
			} else {
				FullscreenType::Off
			})?;

		}

		return Ok(());

	}

	pub fn is_fullscreen(&self) -> bool {
		#[cfg(not(web))] {
			return self.window.fullscreen_state() != sdl2::video::FullscreenType::Off;
		}
		#[cfg(web)] {
			return false;
		}
	}

	pub fn toggle_fullscreen(&mut self) -> Result<()> {
		return self.set_fullscreen(!self.is_fullscreen());
	}

	pub fn set_cursor_hidden(&mut self, b: bool) {
		#[cfg(not(web))] {
			self.sdl_ctx.mouse().show_cursor(b);
		}
	}

	pub fn is_cursor_hidden(&self) -> bool {
		#[cfg(not(web))] {
			return self.sdl_ctx.mouse().is_cursor_showing();
		}
		#[cfg(web)] {
			return false;
		}
	}

	pub fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	pub fn set_cursor_relative(&mut self, b: bool) {
		#[cfg(not(web))] {
			self.sdl_ctx.mouse().set_relative_mouse_mode(b);
		}
	}

	pub fn is_cursor_relative(&self) -> bool {
		return self.sdl_ctx.mouse().relative_mouse_mode();
	}

	pub fn toggle_cursor_relative(&mut self) {
		return self.set_cursor_relative(!self.is_cursor_relative());
	}

	pub fn set_cursor(&self, c: CursorStyle) -> Result<()> {
		#[cfg(not(web))] {
			sdl2::mouse::Cursor::from_system(c)?.set();
		}
		return Ok(());
	}

	pub fn set_custom_cursor(&self, c: &Cursor) {
		#[cfg(not(web))] {
			c.sdl_cursor.set();
		}
	}

	pub fn set_title(&mut self, t: &str) {
		#[cfg(not(web))] {
			self.window.set_title(t);
		}
		#[cfg(web)] {
			stdweb::web::document().set_title(t);
		}
	}

	pub fn title(&self) -> &str {
		return &self.title;
	}

	pub fn dpi(&self) -> f32 {

		#[cfg(not(web))] {

			let (w, h) = self.window.size();
			let (dw, dh) = self.window.drawable_size();

			return dw as f32 / w as f32;

		}

		#[cfg(web)] {
			return 1.0;
		}

	}

	pub fn width(&self) -> i32 {
		return self.width;
	}

	pub fn height(&self) -> i32 {
		return self.height;
	}

	pub fn gwidth(&self) -> f32 {
		return self.conf.width as f32 / self.conf.scale;
	}

	pub fn gheight(&self) -> f32 {
		return self.conf.height as f32 / self.conf.scale;
	}

}

// TODO
pub struct Cursor {
	sdl_cursor: sdl2::mouse::Cursor,
}

pub(super) fn swap(ctx: &app::Ctx) {
	#[cfg(not(web))] {
		ctx.window.gl_swap_window();
	}
}

