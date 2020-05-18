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

#[macro_use]
pub mod utils;

mod gl;
mod window;
mod conf;

#[cfg(not(web))]
mod native;
#[cfg(web)]
mod web;

mod app;
pub use app::*;

pub mod input;
pub mod math;

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;

