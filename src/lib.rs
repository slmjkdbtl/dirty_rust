// wengwengweng

//! # DIRTY
//! simple toolkit for creating game-like experiences

#![allow(unused_parens)]
#![deny(missing_docs)]

#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::collapsible_if)]

#[macro_use]
mod utils;
mod backends;
mod modules;
pub mod addons;
pub mod math;

pub use crate::math::vec::*;
pub use crate::modules::*;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton as Mouse;

