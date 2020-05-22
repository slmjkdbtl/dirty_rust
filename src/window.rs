// wengwengweng

//! Window Events & Operations
//!
//! see methods under [Window](window::Window)

#[cfg(not(web))]
export!(native);
#[cfg(web)]
export!(web);

use crate::*;

pub(crate) enum WindowEvent {
	Input(input::Event),
	Resize(i32, i32),
	DPIChange(f32),
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

