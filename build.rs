// wengwengweng

use std::env;

fn cfg(c: &str) {
	println!("cargo:rustc-cfg={}", c);
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

}


