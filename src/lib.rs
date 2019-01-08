// wengwengweng

//! # DIRTY
//! simple toolkit for creating game-like experiences

#![allow(unused_parens)]

#[macro_use]
mod utils;
mod addons;
mod bindings;
mod modules;
pub mod math;

pub use crate::math::vec::*;
pub use crate::modules::*;
pub use crate::addons::*;
pub use crate::bindings::*;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

