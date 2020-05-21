// wengwengweng

#![feature(clamp)]
#![feature(try_blocks)]
#![feature(box_syntax)]
#![feature(option_zip)]
#![feature(trait_alias)]
#![feature(const_int_pow)]

#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[macro_use]
pub mod utils;
pub mod res;

mod conf;
pub use conf::*;

mod gl;

pub mod app;
pub mod window;
pub mod gfx;
pub mod audio;
pub mod input;

mod run;
pub use run::*;
mod state;
pub use state::*;
mod ctx;
pub use ctx::*;

pub mod fs;
pub mod geom;
pub mod img;
pub mod math;
pub mod codec;
pub mod ui;
pub mod kit;

#[cfg(feature = "midi")]
pub mod midi;
#[cfg(feature = "synth")]
pub mod synth;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "ase")]
pub mod ase;

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;

