// wengwengweng

use std::env;
use std::process;
use std::path::Path;
use std::path::PathBuf;

#[cfg(feature = "lua")]
const DEFAULT_FILE: &'static str = "main.lua";
#[cfg(feature = "python")]
const DEFAULT_FILE: &'static str = "main.py";

#[cfg(any(feature = "lua", feature = "python"))]
fn run(path: Option<impl AsRef<Path>>, args: Option<&[String]>) {

	#[cfg(feature = "lua")]
	let runner = dirty::lua::run;
	#[cfg(feature = "python")]
	let runner = dirty::python::run;

	let path = path
		.map(|s| s.as_ref().to_owned())
		.unwrap_or(PathBuf::from(DEFAULT_FILE));

	#[cfg(feature = "fs")]
	let code = dirty::fs::read_str(&path).ok();
	#[cfg(not(feature = "fs"))]
	let code = std::fs::read_to_string(&path).ok();

	if let Some(code) = code {
		if let Err(err) = runner(&code, Some(&path), args) {
			eprintln!("{}", err);
		}
	} else {
		eprintln!("failed to load {}", path.display());
		process::exit(1);
	}

}

fn main() {

	#[cfg(any(feature = "lua", feature = "python"))] {

		let args = env::args().collect::<Vec<String>>();

		if let Some(action) = args.get(1) {
			run(Some(action), Some(&args[2..args.len()]));
		} else {
			run(None as Option<&str>, None);
		}

	}

}

