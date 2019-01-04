// wengwengweng

use rlua::Result;
use rlua::Lua;
use rlua::Function;
use rlua::UserData;

use crate::*;
use crate::math::*;

fn bind(lua: &Lua) -> Result<()> {

	let globals = lua.globals();

	macro_rules! bind_func {
		($name:expr, ($($args:ident),*): ($($arg_types:ty),*) -> {$code:expr}) => {
			globals.set($name, lua.create_function(|_, ($($args),*): ($($arg_types),*)| {
				return Ok($code);
			})?)?;
		}
	}

	bind_func!("app_init", (): () -> {
		app::init()
	});

	bind_func!("window_init", (title, width, height): (String, u32, u32) -> {
		window::init(&title[..], width, height)
	});

	bind_func!("app_run", (f): (Function) -> {
		app::run(&mut || {
			f.call::<_, ()>(()).expect("something terrible happened");
		})
	});

	bind_func!("gfx_clear", (): () -> {
		gfx::clear()
	});

	return Ok(());

}

pub fn run(code: &str) -> Result<()> {

	let lua = Lua::new();

	bind(&lua)?;
	lua.exec::<_, ()>(code, None)?;

	return Ok(());

}

