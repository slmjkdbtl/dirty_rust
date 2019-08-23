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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Action {
	Link(Lib),
	Search(Path),
	Flags(&'static str),
	Cfg(&'static str),
	Env(&'static str, &'static str),
}

fn perform(ops: Action) {

	use Action::*;

	match ops {

		Link(lib) => {

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

			println!("cargo:rustc-link={}={}", kind, name);

		},

		Search(search) => {

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

			println!("cargo:rustc-search={}={}", kind, path);

		},

		Flags(f) => {
			println!("cargo:rustc-flags={}", f);
		},

		Cfg(c) => {
			println!("cargo:rustc-cfg={}", c);
		},

		Env(k, v) => {
			println!("cargo:rustc-env={}={}", k, v);
		},

	}

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
			perform(Action::Link(Lib::Framework("OpenGLES")));
		}
	}

}

