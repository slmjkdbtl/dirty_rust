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
pub mod lua;

pub use modules::*;

