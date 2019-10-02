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

macro_rules! hashmap {
	($($key:ident => $val:expr),*$(,)?) => {
		let mut _tmp_map = std::collections::HashMap::new();
		$(
			_tmp_map.insert($key, $val);
		)*
		_tmp_map
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

pub mod bindings;
pub mod codec;

mod err;
pub use err::Error;
pub type Result<T> = ::std::result::Result<T, Error>;

