// wengwengweng

//! Window Creation & Config

use gctx::ctx;

use crate::*;
use crate::math::*;

// context
ctx!(WINDOW: WindowCtx);

pub(super) struct WindowCtx {

	sdl_ctx: sdl2::Sdl,
	window: sdl2::video::Window,
	#[allow(dead_code)]
	gl_ctx: sdl2::video::GLContext,
	size: (u32, u32),

}

/// start window with title, width, and height
pub fn init(title: &str, width: u32, height: u32) {

	if !app::enabled() {
		panic!("can't init window without app");
	}

	let sdl_ctx = sdl2::init().expect("failed to init SDL context");
	let video = sdl_ctx.video().expect("failed to init SDL video subsystem");
	let gl_attr = video.gl_attr();

	gl_attr.set_context_profile(sdl2::video::GLProfile::Compatibility);
	gl_attr.set_context_version(2, 1);

	let window = video.window(title, width, height)
		.opengl()
		.resizable()
		.build()
		.expect("failed to create window");

	let gl_ctx = window.gl_create_context().expect("failed to create OpenGL context");

	gl::load_with(|name| {
		video.gl_get_proc_address(name) as *const std::os::raw::c_void
	});

	let events = sdl_ctx.event_pump().expect("failed to create event pump");

	ctx_init(WindowCtx {

		window: window,
		gl_ctx: gl_ctx,
		sdl_ctx: sdl_ctx,
		size: (width, height),

	});

	gfx::init();
	input::init(events);

}

/// check if window is initiated
pub fn enabled() -> bool {
	return ctx_ok();
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

/// set window fullscreen state
pub fn set_fullscreen(b: bool) {

	use sdl2::video::FullscreenType;

	let app_mut = ctx_mut();

	if b {
		app_mut.window.set_fullscreen(FullscreenType::Desktop).expect("fullscreen failed");
	} else {
		app_mut.window.set_fullscreen(FullscreenType::Off).expect("fullscreen failed");
	}

}

/// get window fullscreen state
pub fn get_fullscreen() -> bool {
	use sdl2::video::FullscreenType;
	return ctx_get().window.fullscreen_state() == FullscreenType::Desktop;
}

/// show cursor
pub fn show_cursor() {
	ctx_mut().sdl_ctx.mouse().show_cursor(true);
}

/// hide cursor
pub fn hide_cursor() {
	ctx_mut().sdl_ctx.mouse().show_cursor(false);
}

/// set mouse relative state
pub fn set_relative(b: bool) {
	ctx_mut().sdl_ctx.mouse().set_relative_mouse_mode(b);
}

/// get mouse relative state
pub fn get_relative() -> bool {
	return ctx_get().sdl_ctx.mouse().relative_mouse_mode();
}

/// get view size
pub fn size() -> (u32, u32) {
	return ctx_get().size;
}

pub(super) fn begin() {
	input::poll();
	gfx::begin();
}

pub(super) fn end() {
	gfx::end();
	swap();
}

pub(super) fn swap() {
	ctx_get().window.gl_swap_window();
}

