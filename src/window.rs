// wengwengweng

#[cfg(not(web))]
mod native;
#[cfg(web)]
mod web;

#[cfg(web)]
pub use web::Window;
#[cfg(not(web))]
pub use native::Window;

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

