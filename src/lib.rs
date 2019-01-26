// wengwengweng

//! # DIRTY
//! simple toolkit for creating game-like experiences

#![allow(unused_parens)]
#![allow(dead_code)]

#[macro_use]
mod utils;
mod backends;
mod modules;
pub mod addons;
pub mod math;

pub use math::vec::*;
pub use math::mat::*;
pub use math::rand::*;
pub use modules::*;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

