// wengwengweng

use rlua::Lua;
use rlua::Result;
use rlua::Function;
// use rlua::MetaMethod;
// use rlua::UserData;
// use rlua::UserDataMethods;

use crate::app;
use crate::gfx;
use crate::audio;
use crate::res;
use crate::math;

fn bind(lua: &Lua) -> Result<()> {

	let globals = lua.globals();

	macro_rules! bind_func {
		($name:expr, ($($args:ident),*): ($($arg_types:ty),*) $code:block) => {
			globals.set($name, lua.create_function(|_, ($($args),*): ($($arg_types),*)| {
				$code;
				return Ok(());
			})?)?;
		}
	}

	bind_func!("d_init", (title, width, height): (String, u32, u32) {
		app::init(&title[..], width, height);
	});

	bind_func!("d_run", (f): (Function) {
		app::run(&mut || {
			f.call::<_, ()>(()).expect("something terrible happened");
		});
	});

	bind_func!("d_clear", (): () {
		gfx::clear();
	});

	return Ok(());

}

pub fn run(code: &str) -> Result<()> {

	let lua = Lua::new();

	bind(&lua)?;
	lua.exec::<_>(code, None)?;

	return Ok(());

}


