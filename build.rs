// wengwengweng

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

	let kind;
	let name;

	match lib {
		Static(p) => {
			kind = "static";
			name = p;
		},
		Dylib(p) => {
			kind = "Dylib";
			name = p;
		},
		Framework(p) => {
			kind = "framework";
			name = p;
		},
	}

	println!("cargo:rustc-link-lib={}={}", kind, name);

}

fn search(search: Path) {

	use Path::*;

	let kind;
	let path;

	match search {
		Dependency(p) => {
			kind = "dependency";
			path = p;
		},
		Crate(p) => {
			kind = "crate";
			path = p;
		},
		Native(p) => {
			kind = "native";
			path = p;
		},
		Framework(p) => {
			kind = "framework";
			path = p;
		},
		All(p) => {
			kind = "all";
			path = p;
		},
	}

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

	arch!("wasm32", cfg("web"));
	os!("ios", cfg("mobile"));
	os!("ios", link(Lib::Framework("OpenGLES")));
	os!("macos", cfg("desktop"));
	os!("linux", cfg("desktop"));
	os!("windows", cfg("desktop"));

}

