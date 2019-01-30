// wengwengweng

//! Simple Lua Bindings

use hlua::Lua;

use crate::*;

pub fn run_code(code: &str) {

	let mut lua = Lua::new();

	lua.set("app_init", hlua::function0(|| {
		return app::init();
	}));

	lua.set("window_init", hlua::function3(|t: String, w: u32, h: u32| {
		return window::init(&t, w, h);
	}));

	lua.set("audio_init", hlua::function0(|| {
		return audio::init();
	}));

// 	lua.set("app_run", hlua::function1(|| {
// 		return app::run(&mut || {
			// ...
// 		});
// 	}));

	lua.set("app_fps", hlua::function0(|| {
		return app::fps();
	}));

	lua.execute::<()>(code).expect("failed to run lua");

}

pub fn run(fname: &str) {
	run_code(&fs::read_str(fname));
}

