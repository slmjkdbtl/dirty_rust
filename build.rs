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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum OS {
	Windows,
	MacOS,
	IOS,
	Linux,
	Android,
	FreeBSD,
	DragonFly,
	OpenBSD,
	NetBSD,
}

impl OS {
	fn from_str(s: &str) -> Option<Self> {
		use OS::*;
		return match s {
			"windows" => Some(Windows),
			"macos" => Some(MacOS),
			"ios" => Some(IOS),
			"linux" => Some(Linux),
			"android" => Some(Android),
			"freebsd" => Some(FreeBSD),
			"dragonfly" => Some(DragonFly),
			"openbsd" => Some(OpenBSD),
			"netbsd" => Some(NetBSD),
			_ => None,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Family {
	Windows,
	Unix,
}

impl Family {
	fn from_str(s: &str) -> Option<Self> {
		use Family::*;
		return match s {
			"windows" => Some(Windows),
			"unix" => Some(Unix),
			_ => None,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Arch {
	X86,
	X86_64,
	Mips,
	PowerPC,
	PowerPC64,
	Arm,
	AArch64,
	Wasm32,
}

impl Arch {
	fn from_str(s: &str) -> Option<Self> {
		use Arch::*;
		return match s {
			"x86" => Some(X86),
			"x86_64" => Some(X86_64),
			"mips" => Some(Mips),
			"powerpc" => Some(PowerPC),
			"powerpc64" => Some(PowerPC64),
			"arm" => Some(Arm),
			"aarch64" => Some(AArch64),
			"wasm32" => Some(Wasm32),
			_ => None,
		};
	}
}

fn target_os() -> Option<OS> {
	if let Ok(os) = env::var("CARGO_CFG_TARGET_OS") {
		return OS::from_str(&os);
	} else {
		return None;
	}
}

fn target_family() -> Option<Family> {
	if let Ok(fam) = env::var("CARGO_CFG_TARGET_FAMILY") {
		return Family::from_str(&fam);
	} else {
		return None;
	}
}

fn target_arch() -> Option<Arch> {
	if let Ok(arch) = env::var("CARGO_CFG_TARGET_ARCH") {
		return Arch::from_str(&arch);
	} else {
		return None;
	}
}

fn main() {

	if let Some(os) = target_os() {
		if let OS::IOS = os {
			link(Lib::Framework("OpenGLES"));
		}
	}

}

