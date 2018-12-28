// wengwengweng

use rlua::Lua;
use rlua::Result;
use rlua::Function;
// use rlua::MetaMethod;
// use rlua::UserData;
// use rlua::UserDataMethods;

use dirty::*;

fn bind(lua: &Lua) -> Result<()> {

	let globals = lua.globals();

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

	return Ok(());

}

pub fn code(code: &str) -> Result<()> {

	let lua = Lua::new();

	bind(&lua)?;
	lua.exec::<_>(code, None)?;

	return Ok(());

}

