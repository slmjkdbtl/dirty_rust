// wengwengweng

use std::env;
use dirty::*;

fn main() {

	let args = env::args().collect::<Vec<String>>();

	if let Some(action) = args.get(1) {

		if let Ok(code) = fs::read_str(action) {
			lua::run(&code, Some(action), Some(&args[2..args.len()]));
		} else {
			eprintln!("no");
		}

	} else {

		if let Ok(code) = fs::read_str("main.lua") {
			lua::run(&code, Some("main.lua"), None);
		} else {
			eprintln!("no");
		}

	}

}

