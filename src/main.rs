// wengwengweng

use std::env;
use dirty::*;

fn no() {
	eprintln!("no");
}

fn main() {

	let args = env::args().collect::<Vec<String>>();

	if let Some(action) = args.get(1) {

		if let Ok(code) = fs::read_str(action) {
			if let Err(_) = lua::run(&code, Some(action), Some(&args[2..args.len()])) {
				no();
			}
		} else {
			no();
		}

	} else {

		if let Ok(code) = fs::read_str("main.lua") {
			if let Err(_) = lua::run(&code, Some("main.lua"), None) {
				no();
			}
		} else {
			no();
		}

	}

}

