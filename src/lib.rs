// wengwengweng

//! # DIRTY
//! toolkit for things

#![allow(unused_parens)]
#![allow(dead_code)]

#![deny(clippy::implicit_return)]

pub mod math;
pub mod task;

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

#[cfg(feature = "geom")]
pub mod geom;

#[cfg(feature = "assets")]
pub mod assets;

#[cfg(feature = "ase")]
pub mod ase;

mod bindings;

#[cfg(feature = "lua")]
pub use bindings::lua;

#[cfg(feature = "python")]
pub use bindings::python;

mod err;
pub use err::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

