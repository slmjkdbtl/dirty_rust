// wengwengweng

//! # DIRTY
//! toolkit for things

#![allow(unused_parens)]
#![allow(dead_code)]

#![deny(clippy::implicit_return)]

macro_rules! export {
	($name:ident) => {
		mod $name;
		pub use $name::*;
	}
}

pub mod math;
pub mod task;

#[cfg(feature = "app")]
pub mod gl;
#[cfg(feature = "app")]
pub mod app;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "img")]
pub mod img;

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "geom")]
pub mod geom;

#[cfg(feature = "physics")]
pub mod physics;

#[cfg(feature = "http")]
pub mod http;

#[cfg(all(feature = "term", not(target_os = "ios")))]
pub mod term;

#[cfg(feature = "serial")]
pub mod serial;

#[cfg(feature = "ase")]
pub mod ase;

#[cfg(feature = "texpack")]
pub mod texpack;

mod bindings;

#[cfg(feature = "lua")]
pub use bindings::lua;

#[cfg(feature = "python")]
pub use bindings::python;

mod err;
pub use err::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

