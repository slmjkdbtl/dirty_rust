// wengwengweng

//! # DIRTY
//! Toolkit for Games & Interactive Experiences

#![feature(clamp)]
#![feature(const_fn)]
#![feature(try_blocks)]
#![feature(box_syntax)]
#![feature(option_zip)]
#![feature(trait_alias)]
#![feature(bool_to_option)]
#![feature(fixed_size_array)]
#![feature(type_alias_impl_trait)]

#![allow(unused_parens)]
#![deny(clippy::implicit_return)]

#[macro_use]
pub mod utils;

mod app;
pub use app::*;

mod state;
mod conf;

pub mod gfx;
pub mod ui;
pub mod res;
mod fps;
mod texture;
mod shader;
mod canvas;
mod transform;
mod font;
mod camera;
mod model;
mod desc;
mod skybox;

#[cfg(web)]
mod web;
// #[cfg(not(web))]
// mod native;

#[cfg(not(web))]
pub mod audio;
pub mod input;
pub mod window;
pub mod geom;
pub mod shapes;
pub mod kit;

pub mod task;
pub mod fs;
pub mod math;
pub mod gl;
pub mod img;
pub mod term;
pub mod codec;

#[cfg(feature = "midi")]
pub mod midi;

#[cfg(feature = "synth")]
pub mod synth;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "ase")]
pub mod ase;

#[cfg(feature = "lua")]
mod lua;

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;

