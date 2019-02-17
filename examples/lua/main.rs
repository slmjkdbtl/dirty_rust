// wengwengweng

use dirty::*;
use dirty::addons::lua;

fn main() {
	lua::run_code(include_str!("test.lua"));
}

