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
			lua::run(action);
		} else if action == "export" {
			// ...
		}
	} else {
		if fs::exists("main.lua") {
			lua::run("main.lua");
		} else {
			panic!("no");
		}
	}

}

