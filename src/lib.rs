// wengwengweng

//! # DIRTY
//! toolkit for things

#![allow(unused_parens)]
#![allow(dead_code)]

#![deny(clippy::implicit_return)]

pub mod math;
pub mod thread;

#[cfg(feature = "app")]
pub mod app;

pub use app::window;
pub use app::gfx;

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

