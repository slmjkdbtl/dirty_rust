// wengwengweng

//! Window Events & Operations
//!
//! see methods under [Window](struct.Window.html)

#[cfg(not(web))]
export!(native);
// export!(nativeo);
// export!(sdl);
#[cfg(web)]
export!(web);

use crate::*;

pub(crate) enum WindowEvent {
	Input(input::Event),
	Resize(i32, i32),
	DPIChange(f32),
	Frame,
	Quit,
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

