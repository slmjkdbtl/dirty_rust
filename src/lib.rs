// wengwengweng

//! # DIRTY
//! toolkit for things

#![allow(unused_parens)]
#![allow(dead_code)]

#![deny(clippy::implicit_return)]

mod ggl;
mod modules;
pub mod math;
pub mod thread;
pub mod lua;
pub mod ketos;

mod err;
pub use err::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

pub use modules::*;

