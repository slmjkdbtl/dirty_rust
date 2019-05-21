// wengwengweng

//! Lua Bindings

// wengwengweng

use std::path::Path;
use crate::*;

use rlua::Lua;
use rlua::Result;

pub fn run(code: &str) -> Result<()> {

	let lua = Lua::new();

	return lua.context(|ctx| {

		let globals = ctx.globals();
		let fs = ctx.create_table()?;
		let win = ctx.create_table()?;

		fs.set("glob", ctx.create_function(|_, (pat): (String)| {
			return Ok(fs::glob(&pat));
		})?)?;

		fs.set("exists", ctx.create_function(|_, (fname): (String)| {
			return Ok(fs::exists(&fname));
		})?)?;

		fs.set("read_str", ctx.create_function(|_, (fname): (String)| {
			return Ok(fs::read_str(&fname));
		})?)?;

		fs.set("read_bytes", ctx.create_function(|_, (fname): (String)| {
			return Ok(fs::read_bytes(&fname));
		})?)?;

		fs.set("basename", ctx.create_function(|_, (fname): (String)| {
			return Ok(fs::basename(&fname));
		})?)?;

		globals.set("fs", fs)?;
		globals.set("win", win)?;
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

