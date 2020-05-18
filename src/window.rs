// wengwengweng

use std::rc::Rc;

use crate::*;
use math::*;
use input::*;

pub enum WindowEvent {
	Input(input::Event),
	Frame,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorIcon {
	Normal,
	Hand,
	Cross,
	Move,
	Progress,
	Wait,
	Text,
}

pub trait WindowCtx {

	fn run(self, _: impl FnMut(&mut Self, WindowEvent) -> Result<bool> + 'static) -> Result<()>;

	fn gl(&self) -> &Rc<gl::Device>;
	fn swap(&self) -> Result<()>;

	fn key_down(&self, _: Key) -> bool;
	fn mouse_down(&self, _: Mouse) -> bool;

	fn key_mods(&self) -> KeyMod {
		return KeyMod {
			shift: self.key_down(Key::LShift) || self.key_down(Key::RShift),
			ctrl: self.key_down(Key::LCtrl) || self.key_down(Key::RCtrl),
			alt: self.key_down(Key::LAlt) || self.key_down(Key::RAlt),
			meta: self.key_down(Key::LMeta) || self.key_down(Key::RMeta),
		};
	}

	fn width(&self) -> i32;
	fn height(&self) -> i32;

	fn dpi(&self) -> f32;

	fn mouse_pos(&self) -> Vec2;
	fn set_mouse_pos(&mut self, _: Vec2) -> Result<()>;

	fn clip_to_screen(&self, p: Vec2) -> Vec2 {
		return p * vec2!(self.width(), self.height()) * 0.5;
	}

	fn screen_to_clip(&self, p: Vec2) -> Vec2 {
		return p / 0.5 / vec2!(self.width(), self.height());
	}

	fn set_fullscreen(&mut self, _: bool);
	fn is_fullscreen(&self) -> bool;
	fn toggle_fullscreen(&mut self) {
		self.set_fullscreen(!self.is_fullscreen());
	}

	fn set_cursor_hidden(&mut self, _: bool);
	fn is_cursor_hidden(&self) -> bool;
	fn toggle_cursor_hidden(&mut self) {
		self.set_cursor_hidden(!self.is_cursor_hidden());
	}

	fn set_cursor_locked(&mut self, _: bool);
	fn is_cursor_locked(&self) -> bool;
	fn toggle_cursor_locked(&mut self) {
		self.set_cursor_locked(!self.is_cursor_locked());
	}

	fn set_title(&mut self, _: &str);
	fn title(&self) -> &str;

}

