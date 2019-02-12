// wengwengweng

//! Lua Bindings

use rlua::Lua;
use rlua::Function;
use rlua::UserData;
use rlua::UserDataMethods;
use rlua::MetaMethod;

use crate::*;
use crate::math::*;

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

		impl UserData for Vec2 {

			fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

				methods.add_method("mag", |_, v, ()| {
					return Ok(Vec2::mag(&v));
				});

				methods.add_meta_function(MetaMethod::Add, |_, (p1, p2): (Vec2, Vec2)| {
					return Ok(p1 + p2);
				});

				methods.add_meta_function(MetaMethod::Mul, |_, (p, s): (Vec2, f32)| {
					return Ok(p * s);
				});

				methods.add_meta_function(MetaMethod::ToString, |_, (p): (Vec2)| {
					return Ok(format!("{}", p));
				});

			}
		}

		impl UserData for Rect {}
		impl UserData for Color {}
		impl UserData for gfx::Texture {}

		bind_func!("vec2", (x, y): (f32, f32) -> vec2!(x, y));
		bind_func!("color", (r, g, b, a): (f32, f32, f32, f32) -> color!(r, g, b, a));
		bind_func!("rect", (x, y, w, h): (f32, f32, f32, f32) -> rect!(x, y, w, h));

		bind_func!("app_init", (): () -> app::init());
		bind_func!("app_enabled", (): () -> app::enabled());
		bind_func!("app_quit", (): () -> app::quit());
		bind_func!("app_time", (): () -> app::time());
		bind_func!("app_dt", (): () -> app::dt());
		bind_func!("app_fps", (): () -> app::fps());
		bind_func!("app_set_debug", (b): (bool) -> app::set_debug(b));
		bind_func!("app_debug", (): () -> app::debug());
		bind_func!("app_is_macos", (): () -> app::is_macos());
		bind_func!("app_is_windows", (): () -> app::is_windows());
		bind_func!("app_is_linux", (): () -> app::is_linux());
		bind_func!("app_is_ios", (): () -> app::is_ios());
		bind_func!("app_is_android", (): () -> app::is_android());

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

		bind_func!("gfx_clear", (): () -> gfx::clear());
		bind_func!("gfx_reset", (): () -> g2d::reset());

		ctx.load(code).exec().expect("failed to run lua");

	});

}

/// run from lua file
pub fn run(fname: &str) {
	return run_code(&fs::read_str(fname));
}

