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

pub mod window;
#[macro_use]
pub mod audio;
pub mod http;
pub mod fs;
pub mod data;
pub mod col;
pub mod img;
pub mod lua;
pub mod err;

pub use err::Error;

