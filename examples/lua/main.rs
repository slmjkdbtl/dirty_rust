// wengwengweng

use sock::*;
use sock::addons::lua;

fn main() {
	lua::run_code(include_str!("test.lua"));
}

