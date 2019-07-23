// wengwengweng

//! # DIRTY
//! toolkit for things

#![allow(unused_parens)]
#![allow(dead_code)]

#![deny(clippy::implicit_return)]

mod modules;

pub mod math;
pub mod thread;

#[cfg(feature = "lua")]
pub mod lua;

mod err;
pub use err::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

pub use modules::*;

