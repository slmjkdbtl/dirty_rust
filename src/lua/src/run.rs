// wengwengweng

use rlua::Lua;
use rlua::Result;
use rlua::Function;
// use rlua::MetaMethod;
// use rlua::UserData;
// use rlua::UserDataMethods;

use std::fs::File;
use std::io::Read;

use dirty::*;

fn bind(lua: &Lua) -> Result<()> {

	let globals = lua.globals();

	globals.set("string_var", "hello")?;
	globals.set("int_var", 42)?;

	globals.set("d_init", lua.create_function(|_, (title, width, height): (String, u32, u32)| {
		app::init(&title[..], width, height);
		return Ok(());
	})?)?;

	globals.set("d_run", lua.create_function(|_, f: Function| {
		app::run(&mut || {
			f.call::<_, ()>(()).expect("something terrible happened");
		});
		return Ok(());
	})?)?;

	globals.set("d_clear", lua.create_function(|_, (): ()| {
		gfx::clear();
		return Ok(());
	})?)?;

	globals.set("d_line", lua.create_function(|_, (): ()| {
		// ...
		return Ok(());
	})?)?;

	return Ok(());

}

pub fn code(code: &str) -> Result<()> {

	let lua = Lua::new();

	bind(&lua)?;
	lua.exec::<_>(code, None)?;

	return Ok(());

}

fn fread(fname: &str) -> String {

	let mut file = File::open(fname).expect("no file");
	let mut contents = String::new();

	file.read_to_string(&mut contents).expect("cannot read file");

	return contents;

}

pub fn file(fname: &str) -> Result<()> {

	let lua = Lua::new();

	bind(&lua)?;
	lua.exec::<_>(&fread(fname)[..], None)?;

	return Ok(());

}

