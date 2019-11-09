// wengwengweng

//! # DIRTY
//! toolkit for things

#![feature(clamp)]
#![feature(const_fn)]
#![feature(try_blocks)]
#![feature(box_syntax)]
#![feature(trait_alias)]
#![feature(bool_to_option)]
#![feature(fixed_size_array)]
#![feature(type_alias_impl_trait)]

// #![warn(missing_docs)]
#![allow(unused_parens)]
#![deny(clippy::implicit_return)]

#[macro_use]
mod helpers;
mod res;

pub use helpers::lstatic;

pub mod task;

#[cfg(feature = "app")]
pub mod math;
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

#[cfg(all(feature = "ui"))]
pub mod ui;

#[cfg(all(feature = "term", desktop))]
pub mod term;

#[cfg(all(feature = "serial", desktop))]
pub mod serial;

#[cfg(feature = "ase")]
pub mod ase;

#[cfg(feature = "texpack")]
pub mod texpack;

#[cfg(feature = "clip")]
pub mod clip;

pub mod codec;

mod err;
pub use err::Error;
pub type Result<T> = std::result::Result<T, Error>;

