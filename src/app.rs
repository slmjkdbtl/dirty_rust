// wengwengweng

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

struct AppCtx {

	sdl_ctx: sdl2::Sdl,
	window: sdl2::video::Window,

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

	let ctx = window.gl_create_context().unwrap();

	gl::load_with(|name| {
		video.gl_get_proc_address(name) as *const std::os::raw::c_void
	});

}

pub fn run(f: fn()) {
	f();
}

