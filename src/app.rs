// wengwengweng

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::thread;
use std::time;

use crate::*;

ctx!(APP: AppCtx);

struct AppCtx {

	sdl_ctx: sdl2::Sdl,
	window: sdl2::video::Window,
	gl_ctx: sdl2::video::GLContext,

}

pub fn init(title: &str, width: u32, height: u32) {

	let sdl_ctx = sdl2::init().unwrap();
	let video = sdl_ctx.video().unwrap();
	let gl_attr = video.gl_attr();

	gl_attr.set_context_profile(GLProfile::Compatibility);
	gl_attr.set_context_version(2, 1);

	let window = video.window(title, width, height)
		.opengl()
		.build()
		.unwrap();

	let gl_ctx = window.gl_create_context().unwrap();

	gl::load_with(|name| {
		video.gl_get_proc_address(name) as *const std::os::raw::c_void
	});

	gfx::init();
	#[cfg(not(target_os = "windows"))]
	audio::init();
	res::init();

	init_ctx(AppCtx {
		sdl_ctx: sdl_ctx,
		window: window,
		gl_ctx: gl_ctx,
	});

}

pub fn run(f: &mut FnMut()) {

	let app = get_ctx();
	let mut event_pump = app.sdl_ctx.event_pump().unwrap();

	'running: loop {

		gfx::update();
		f();
		app.window.gl_swap_window();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running;
				},
				_ => {}
			}
		}

		thread::sleep(time::Duration::from_millis(16));

	}

}

pub fn size() -> (u32, u32) {
	return get_ctx().window.size();
}

