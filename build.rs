// wengwengweng

#![allow(dead_code)]

use std::env;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Lib {
	Dylib(&'static str),
	Static(&'static str),
	Framework(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Path {
	Dependency(&'static str),
	Crate(&'static str),
	Native(&'static str),
	Framework(&'static str),
	All(&'static str),
}

fn link(lib: Lib) {

	use Lib::*;

	let (kind, name) = match lib {
		Static(p) => ("static", p),
		Dylib(p) => ("dylib", p),
		Framework(p) => ("framework", p),
	};

	println!("cargo:rustc-link-lib={}={}", kind, name);

}

fn search(search: Path) {

	use Path::*;

	let (kind, path) = match search {
		Dependency(p) => ("dependency", p),
		Crate(p) => ("crate", p),
		Native(p) => ("native", p),
		Framework(p) => ("framework", p),
		All(p) => ("all", p),
	};

	println!("cargo:rustc-link-search={}={}", kind, path);

}

fn flags(f: &str) {
	println!("cargo:rustc-flags={}", f);
}

fn cfg(c: &str) {
	println!("cargo:rustc-cfg={}", c);
}

fn env(k: &str, v: &str) {
	println!("cargo:rustc-env={}={}", k, v);
}

macro_rules! arch {
	($target:expr, $action:expr) => {
		if let Ok(arch) = env::var("CARGO_CFG_TARGET_ARCH") {
			if arch == $target {
				$action
			}
		}
	}
}

macro_rules! os {
	($target:expr, $action:expr) => {
		if let Ok(arch) = env::var("CARGO_CFG_TARGET_OS") {
			if arch == $target {
				$action
			}
		}
	}
}

fn main() {

	os!("macos", cfg("desktop"));
	os!("linux", cfg("desktop"));
	os!("windows", cfg("desktop"));
	os!("android", cfg("mobile"));
	os!("ios", cfg("mobile"));

	arch!("wasm32", cfg("web"));
	os!("macos", cfg("macos"));
	os!("linux", cfg("linux"));
	os!("windows", cfg("windows"));
	os!("ios", cfg("ios"));
	os!("android", cfg("android"));

	os!("ios", link(Lib::Framework("OpenGLES")));

}

