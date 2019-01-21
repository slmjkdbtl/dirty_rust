// wengwengweng

//! Lua Bindings

use rlua::Lua;
use rlua::Function;
use rlua::UserData;
// use rlua::UserDataMethods;
// use rlua::MetaMethod;

use crate::*;

/// run from lua code
pub fn run_code(code: &str) {

	let lua = Lua::new();

	lua.context(|ctx| {

		let globals = ctx.globals();

		macro_rules! bind_func {

			($name:expr, ($($args:ident),*): ($($arg_types:ty),*) -> $code:block) => {
				globals.set($name, ctx.create_function(|_, ($($args),*): ($($arg_types),*)|
					$code
				).expect("failed to create lua function")).expect("failed to create lua function");
			};

			($name:expr, ($($args:ident),*): ($($arg_types:ty),*) -> $code:expr) => {
				globals.set($name, ctx.create_function(|_, ($($args),*): ($($arg_types),*)| {
					return Ok($code);
				}).expect("failed to create lua function")).expect("failed to create lua function");
			};

		}

		impl UserData for Vec2 {}
		impl UserData for Rect {}
		impl UserData for Color {}
		impl UserData for gfx::Texture {}

		bind_func!("vec2", (x, y): (f32, f32) -> vec2!(x, y));
		bind_func!("color", (r, g, b, a): (f32, f32, f32, f32) -> color!(r, g, b, a));
		bind_func!("rect", (x, y, w, h): (f32, f32, f32, f32) -> rect!(x, y, w, h));

		bind_func!("app_init", (): () -> app::init());
		bind_func!("app_quit", (): () -> app::quit());
		bind_func!("app_time", (): () -> app::time());
		bind_func!("app_dt", (): () -> app::dt());
		bind_func!("app_fps", (): () -> app::fps());

		bind_func!("app_run", (f): (Function) ->
			app::run(&mut || {
				if f.call::<_, ()>(()).is_err() {
					panic!("failed to run");
				}
			})
		);

		bind_func!("window_init", (title, width, height): (String, u32, u32) -> window::init(&title, width, height));
		bind_func!("window_size", (): () -> window::size());
		bind_func!("window_set_fullscreen", (b): (bool) -> window::set_fullscreen(b));
		bind_func!("window_get_fullscreen", (): () -> window::get_fullscreen());
		bind_func!("window_set_relative", (b): (bool) -> window::set_relative(b));
		bind_func!("window_get_relative", (): () -> window::get_relative());

		bind_func!("gfx_init", (): () -> gfx::init());
		bind_func!("gfx_clear", (): () -> gfx::clear());
		bind_func!("gfx_reset", (): () -> gfx::reset());

		ctx.load(code).exec().expect("failed to run lua");

	});

}

/// run from lua file
pub fn run(fname: &str) {
	run_code(&fs::read_str(fname));
}

