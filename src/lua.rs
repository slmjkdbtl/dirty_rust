// wengwengweng

//! lua bindings

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

	bind_func!("app_quit", (): () -> {
		app::quit()
	});

	bind_func!("app_time", (): () -> {
		app::time()
	});

	bind_func!("app_dt", (): () -> {
		app::dt()
	});

	bind_func!("app_fps", (): () -> {
		app::fps()
	});

	bind_func!("app_error", (msg): (String) -> {
		app::error(&msg)
	});

	bind_func!("app_run", (f): (Function) -> {
		app::run(&mut || {
			f.call::<_, ()>(()).expect("something terrible happened");
		})
	});

	bind_func!("window_init", (title, width, height): (String, u32, u32) -> {
		window::init(&title, width, height)
	});

	bind_func!("window_size", (): () -> {
		window::size()
	});

	bind_func!("window_set_fullscreen", (b): (bool) -> {
		window::set_fullscreen(b)
	});

	bind_func!("window_get_fullscreen", (): () -> {
		window::get_fullscreen()
	});

	bind_func!("window_set_relative", (b): (bool) -> {
		window::set_relative(b)
	});

	bind_func!("window_get_relative", (): () -> {
		window::get_relative()
	});

	bind_func!("gfx_init", (): () -> {
		gfx::init()
	});

	bind_func!("gfx_clear", (): () -> {
		gfx::clear()
	});

	bind_func!("gfx_reset", (): () -> {
		gfx::reset()
	});

	return Ok(());

}

pub fn run(code: &str) -> Result<()> {

	let lua = Lua::new();

	bind(&lua)?;
	lua.exec::<_, ()>(code, None)?;

	return Ok(());

}

