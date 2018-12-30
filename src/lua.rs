// wengwengweng

use rlua::Lua;
use rlua::Result;
use rlua::Function;
use rlua::UserData;
// use rlua::UserDataMethods;
// use rlua::MetaMethod;

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

	bind_func!("d_init", (title, width, height): (String, u32, u32) -> {
		app::init(&title[..], width, height)
	});

	bind_func!("d_run", (f): (Function) -> {
		app::run(&mut || {
			f.call::<_, ()>(()).expect("something terrible happened");
		})
	});

	bind_func!("d_clear", (): () -> {
		gfx::clear()
	});

	impl UserData for Vec2 {}

	bind_func!("vec2", (x, y): (f32, f32) -> {
		vec2!(x, y)
	});

	return Ok(());

}

pub fn run(code: &str) -> Result<()> {

	let lua = Lua::new();

	bind(&lua)?;
	lua.exec::<_>(code, None)?;

	return Ok(());

}


