// wengwengweng

//! Lua Bindings

// waiting for https://github.com/kyren/luster to be in a usable state

use std::path::Path;

use crate::*;

pub fn run_code(code: &str) {
	unimplemented!();
}

pub fn run(fname: impl AsRef<Path>) {
	run_code(&fs::read_str(fname));
}

