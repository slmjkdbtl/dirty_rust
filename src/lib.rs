// wengwengweng

//! # DIRTY
//! toolkit for things

#![allow(unused_parens)]

#![deny(clippy::implicit_return)]

#[allow(unused_macros)]
macro_rules! export {
	($name:ident) => {
		mod $name;
		pub use $name::*;
	}
}

#[allow(unused_macros)]
macro_rules! hashmap {
	($($key:expr => $val:expr),*$(,)?) => {
		{
		let mut _tmp = std::collections::HashMap::new();
		$(_tmp.insert($key, $val);)*
		_tmp
		}
	}
}

#[allow(unused_macros)]
macro_rules! hashset {
	($($item:expr),*$(,)?) => {
		{
			let mut _tmp = std::collections::HashSet::new();
			$(_tmp.insert($item);)*
			_tmp
		}
	};
}

mod res;

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
pub type Result<T> = std::result::Result<T, Error>;

