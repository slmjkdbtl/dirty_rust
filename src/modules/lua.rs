// wengwengweng

//! Lua Bindings

// wengwengweng

use std::path::Path;

use rlua::Lua;
use rlua::Result;

pub fn run(code: &str) -> Result<()> {

	let lua = Lua::new();

	return lua.context(|ctx| {

		let globals = ctx.globals();
		let fs = ctx.create_table()?;

		globals.set("fs", fs)?;
		ctx.load(code).exec()?;

		return Ok(());

	});

}

pub fn run_file(path: impl AsRef<Path>) {

	let path = path.as_ref();

	if let Ok(code) = std::fs::read_to_string(path) {
		run(&code);
	} else {
		panic!("failed to read {}", path.display());
	}

}

