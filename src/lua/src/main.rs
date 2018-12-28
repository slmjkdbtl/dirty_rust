// wengwengweng

#![allow(unused_parens)]
#![allow(dead_code)]

use std::env;

mod run;
mod export;
mod utils;

fn main() {

	if let Some(action) = env::args().nth(1) {
		if (action == "export") {
			export::macos();
		} else {
			if let Ok(content) = utils::fread(&action[..]) {
				run::code(&content[..]).expect("oh no");
			}
		}
	} else {
		if let Ok(content) = utils::fread("main.lua") {
			run::code(&content[..]).expect("oh no");
		}
	}

}

