// wengwengweng

//! # DIRTY
//! simple toolkit for creating game-like experiences

#![crate_name = "dirty"]
#![deny(clippy::all)]
#![deny(clippy::implicit_return)]
#![deny(clippy::shadow_same)]

#[macro_use]
mod utils;
mod addons;
mod bindings;
mod modules;

pub use crate::modules::*;
pub use crate::addons::*;
pub use crate::bindings::*;
pub use crate::math::*;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

