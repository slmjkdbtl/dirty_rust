// wengwengweng

#![allow(unused_parens)]
#![allow(dead_code)]

use std::env;

mod run;
mod export;

fn main() {

	if let Some(action) = env::args().nth(1) {

		if (action == "export") {
			export::macos();
		} else {
			run::file(&action[..]).expect("oh no");
		}

	} else {
		println!("no");
	}

}

