// wengwengweng

use std::env;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Lib {
	Dylib,
	Static,
	Framework,
}

impl Lib {
	fn as_str(&self) -> &'static str {
		use Lib::*;
		return match self {
			Dylib => "dylib",
			Static => "static",
			Framework => "framework",
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Search {
	Dependency,
	Crate,
	Native,
	Framework,
	All,
}

impl Search {
	fn as_str(&self) -> &'static str {
		use Search::*;
		return match self {
			Dependency => "dependency",
			Crate => "crate",
			Native => "native",
			Framework => "framework",
			All => "all",
		};
	}
}

fn link(kind: Lib, name: &str) {
	println!("cargo:rustc-link-lib={}={}", kind.as_str(), name);
}

fn search(kind: Search, path: impl AsRef<Path>) {
	println!("cargo:rustc-link-search={}={}", kind.as_str(), path.as_ref().display());
}

fn flags(f: &str) {
	println!("cargo:rustc-flags={}", f);
}

fn cfg(f: &str) {
	println!("cargo:rustc-cfg={}", f);
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

fn target_os() -> Option<OS> {
	if let Ok(target_os) = env::var("CARGO_CFG_TARGET_OS") {
		return OS::from_str(&target_os);
	} else {
		return None;
	}
}

fn main() {

	if let Some(os) = target_os() {
		if os == OS::IOS {
			link(Lib::Framework, "OpenGLES");
		}
	}

}

