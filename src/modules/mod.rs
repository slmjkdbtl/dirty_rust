// wengwengweng

macro_rules! expose {
	($state:ident, $fname:ident($($argn:ident: $argt:ty),*)$( -> $return:ty)?) => {
		pub fn $fname($($argn: $argt),*)$( -> $return)? {
			return ctx_get!($state).$fname($($argn),*);
		}
	};
	($state:ident(mut), $fname:ident($($argn:ident: $argt:ty),*)$( -> $return:ty)?) => {
		pub fn $fname($($argn: $argt),*)$( -> $return)? {
			return ctx_mut!($state).$fname($($argn),*);
		}
	};
}

pub mod app;
pub mod window;
#[macro_use]
pub mod gfx;
pub mod g2d;
pub mod g3d;
pub mod audio;
pub mod net;
pub mod fs;
pub mod data;
pub mod col;
pub mod res;
pub mod lua;

