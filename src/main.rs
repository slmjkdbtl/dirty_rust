// wengwengweng

use std::env;


fn no() {
	eprintln!("no");
}

#[cfg(feature = "lua")]
fn run_lua() {

	use dirty::lua;

	let args = env::args().collect::<Vec<String>>();

	if let Some(action) = args.get(1) {

		if let Ok(code) = std::fs::read_to_string(action) {
			if let Err(_) = lua::run(&code, Some(action), Some(&args[2..args.len()])) {
				no();
			}
		} else {
			no();
		}

	} else {

		#[cfg(feature = "fs")]
		let code = dirty::fs::read_str("main.lua");

		#[cfg(not(feature = "fs"))]
		let code = std::fs::read_to_string("main.lua");

		if let Ok(code) = code {
			if let Err(_) = lua::run(&code, Some("main.lua"), None) {
				no();
			}
		} else {
			no();
		}

	}

}

fn main() {

	#[cfg(feature = "lua")]
	run_lua();

}

