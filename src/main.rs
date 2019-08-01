// wengwengweng

use std::env;

fn run() {

	#[cfg(feature = "lua")]
	let runner = dirty::lua::run;

	#[cfg(feature = "python")]
	let runner = dirty::python::run;

	#[cfg(feature = "lua")]
	let default_file = "main.lua";

	#[cfg(feature = "python")]
	let default_file = "main.py";

	let args = env::args().collect::<Vec<String>>();

	if let Some(action) = args.get(1) {

		if let Ok(code) = std::fs::read_to_string(action) {
			if let Err(err) = runner(&code, Some(action), Some(&args[2..args.len()])) {
				eprintln!("{}", err);
			}
		} else {
			eprintln!("failed to read {}", action);
		}

	} else {

		#[cfg(feature = "fs")]
		let code = dirty::fs::read_str(default_file);

		#[cfg(not(feature = "fs"))]
		let code = std::fs::read_to_string(default_file);

		if let Ok(code) = code {
// 			if let Err(err) = runner(&code, Some(default_file), None) {
// 				eprintln!("{}", err);
// 			}
		} else {
			eprintln!("no file to run");
		}

	}

}

fn main() {
	run();
}

