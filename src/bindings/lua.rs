// wengwengweng

//! Lua Bindings

use rlua::Result;
use rlua::Lua;
use rlua::Function;
use rlua::UserData;
// use rlua::UserDataMethods;
// use rlua::MetaMethod;

use crate::*;

fn bind(lua: &Lua) -> Result<()> {

	let globals = lua.globals();

	macro_rules! bind_func {

		($name:expr, ($($args:ident),*): ($($arg_types:ty),*) -> $code:block) => {
			globals.set($name, lua.create_function(|_, ($($args),*): ($($arg_types),*)|
				$code
			)?)?;
		};

		($name:expr, ($($args:ident),*): ($($arg_types:ty),*) -> $code:expr) => {
			globals.set($name, lua.create_function(|_, ($($args),*): ($($arg_types),*)|{
				return Ok($code);
			})?)?;
		};

	}

	impl UserData for Vec2 {}
	impl UserData for Rect {}
	impl UserData for Color {}
	impl UserData for gfx::Texture {}
	impl UserData for res::SpriteData {}
	impl UserData for res::Anim {}
	impl UserData for res::AnimDir {}

	bind_func!("vec2", (x, y): (f32, f32) -> vec2!(x, y));
	bind_func!("color", (r, g, b, a): (f32, f32, f32, f32) -> color!(r, g, b, a));
	bind_func!("rect", (x, y, w, h): (f32, f32, f32, f32) -> rect!(x, y, w, h));

	bind_func!("app_init", (): () -> app::init());
	bind_func!("app_quit", (): () -> app::quit());
	bind_func!("app_time", (): () -> app::time());
	bind_func!("app_dt", (): () -> app::dt());
	bind_func!("app_fps", (): () -> app::fps());
	bind_func!("app_error", (msg): (String) -> app::error(&msg));

	bind_func!("app_run", (f): (Function) ->
		app::run(&mut || {
			if f.call::<_, ()>(()).is_err() {
				app::error("failed to run");
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

	bind_func!("res_init", (): () -> res::init());
	bind_func!("res_load_sprites", (dir, names): (String, Vec<String>) -> res::load_sprites(&dir, &names.iter().map(|s| s.as_ref()).collect()));

// 	bind_func!("gfx_draw", (tex, rect): (&gfx::Texture, Rect) -> {
// 		return Ok(gfx::draw(&tex, rect));
// 	});

// 	bind_func!("res_sprite", (name): (String) -> {
// 		return Ok(res::sprite(&name));
// 	});

	return Ok(());

}

pub fn run_code(code: &str) {

	let lua = Lua::new();

	if bind(&lua).is_err() {
		app::error("failed to bind lua");
	}

	if lua.exec::<_, ()>(code, None).is_err() {
		app::error("failed to run lua");
	}

}

pub fn run(fname: &str) {
	run_code(&fs::read_str(fname));
}

