// wengwengweng

use std::env;
use std::path::Path;
use std::path::PathBuf;

#[cfg(feature = "lua")]
const DEFAULT_FILE: &'static str = "main.lua";
#[cfg(feature = "python")]
const DEFAULT_FILE: &'static str = "main.py";

fn run(path: Option<impl AsRef<Path>>, args: Option<&[String]>) {

	#[cfg(feature = "lua")]
	let runner = dirty::lua::run;
	#[cfg(feature = "python")]
	let runner = dirty::python::run;

	let path: PathBuf = {

		if let Some(p) = path {
			p.as_ref().to_owned()
		} else {
			PathBuf::from(DEFAULT_FILE)
		}

	};

	let code: Option<String> = {
		if cfg!(feature = "fs") {
			dirty::fs::read_str(&path).ok()
		} else {
			std::fs::read_to_string(&path).ok()
		}
	};

	if let Some(code) = code {
		if let Err(err) = runner(&code, Some(&path), None) {
			eprintln!("{}", err);
		}
	} else {
		eprintln!("failed to load {}", path.display());
		std::process::exit(1);
	}

}

fn main() {

	let args = env::args().collect::<Vec<String>>();

	if let Some(action) = args.get(1) {
		run(Some(action), Some(&args[2..args.len()]));
	} else {
		run(None as Option<&str>, None);
	}

}

