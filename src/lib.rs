// wengwengweng

//! # DIRTY
//! toolkit for things

#![allow(unused_parens)]
#![allow(dead_code)]

#![deny(clippy::implicit_return)]

#[cfg(all(feature = "lua", feature = "python"))]
compile_error!("can only enable one scripting option");

pub mod math;
pub mod thread;

#[cfg(feature = "app")]
pub mod app;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "img")]
pub mod img;

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "term")]
pub mod term;

#[cfg(feature = "col")]
pub mod col;

#[cfg(feature = "ase")]
pub mod ase;

#[cfg(feature = "lua")]
pub mod lua;

mod err;
pub use err::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

