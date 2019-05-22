// wengwengweng

use std::env;
use dirty::*;

mod cli;

fn main() {

	ezpanic::set(|info: ezpanic::ErrorInfo| {
		if let Some(message) = &info.message {
			eprintln!("{}", message);
		}
	});

	let args = env::args().collect::<Vec<String>>();

	if let Some(action) = args.get(1) {
		if fs::exists(action) {
			lua::run_file(action, Some(&args[2..args.len()]));
		} else if action == "export" {
			// ...
		} else {
			panic!("not found");
		}
	} else {
		if fs::exists("main.lua") {
			lua::run(&fs::read_str("main.lua"), Some("main.lua"), Some(&args[2..args.len()]));
		} else {
			panic!("no");
		}
	}

}

