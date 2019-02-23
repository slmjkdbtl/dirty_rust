// wengwengweng

//! # DIRTY
//! simple toolkit for creating game-like experiences

#![allow(unused_parens)]
#![allow(dead_code)]

#![deny(clippy::implicit_return)]

#[macro_use]
mod utils;
mod ggl;
mod modules;
pub mod math;

pub use modules::*;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

