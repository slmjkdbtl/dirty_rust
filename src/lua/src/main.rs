// wengwengweng

#![allow(unused_parens)]
#![allow(dead_code)]

use rlua::Lua;
use rlua::Result;
use rlua::Function;
use rlua::MetaMethod;
use rlua::UserData;
use rlua::UserDataMethods;
use rlua::Variadic;

use dirty::*;

fn main() -> Result<()> {

	let lua = Lua::new();

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

	lua.exec::<_>(r#"
		d_init("yo", 640, 480)
		d_run(function()
			d_clear()
			d_line()
		end)
	"#, None)?;

	return Ok(());

}

