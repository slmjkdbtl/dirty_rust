// wengwengweng

macro_rules! expose {
	($state:ident, $fn:ident($($argn:ident: $argt:ty),*) -> $return:ty) => {
		pub fn $fn($($argn: $argt),*) -> $return {
			return ctx_get!($state).$fn($($argn),*);
		}
	};
	($state:ident(mut), a, $fn:ident($($argn:ident: $argt:ty),*) -> $return:ty) => {
		pub fn $fn($($argn: $argt),*) -> $return {
			return ctx_mut!($state).$fn($($argn),*);
		}
	};
	($state:ident, $fn:ident($($argn:ident: $argt:ty),*)) => {
		pub fn $fn($($argn: $argt),*) {
			return ctx_get!($state).$fn($($argn),*);
		}
	};
	($state:ident(mut), $fn:ident($($argn:ident: $argt:ty),*)) => {
		pub fn $fn($($argn: $argt),*) {
			return ctx_mut!($state).$fn($($argn),*);
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

