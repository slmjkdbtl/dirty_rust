// wengwengweng

use std::time::Duration;

use instant::Instant;

use crate::*;
use input::*;
use math::*;
use fps::*;
use window::WindowCtx;
pub use conf::*;

pub struct Ctx<'a> {

	#[cfg(not(web))]
	pub(crate) window: &'a mut native::Window,
	#[cfg(web)]
	pub(crate) window: &'a mut web::Window,

	pub(crate) gfx: &'a mut gfx::GfxCtx,

	time: Duration,
	quit: bool,
	fps_counter: &'a mut FPSCounter,

}

pub trait State: 'static + Sized {

	fn init(_: &mut Ctx) -> Result<Self>;

	fn event(&mut self, _: &mut Ctx, _: &input::Event) -> Result<()> {
		return Ok(());
	}

	fn update(&mut self, _: &mut Ctx, _: Duration) -> Result<()> {
		return Ok(());
	}

	fn draw(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

}

impl State for () {
	fn init(_: &mut Ctx) -> Result<Self> {
		return Ok(());
	}
}

pub trait HasGLDevice {
	fn device(&self) -> &gl::Device;
}

impl<'a> HasGLDevice for Ctx<'a> {
	fn device(&self) -> &gl::Device {
		return self.gfx.device();
	}
}

impl Launcher {
	pub fn run<S: State>(self) -> Result<()> {
		return run_with_conf::<S>(self.conf);
	}
}

pub fn launcher() -> Launcher {
	return Launcher::default();
}

pub fn run<S: State>() -> Result<()> {
	return launcher().run::<S>();
}

fn run_with_conf<S: State>(conf: conf::Conf) -> Result<()> {

	#[cfg(not(web))]
	let mut window = native::Window::new(&conf)?;
	#[cfg(web)]
	let mut window = web::Window::new(&conf)?;

	let mut gfx = gfx::GfxCtx::new(&window, &conf)?;

	window.swap()?;

	let mut time = Duration::from_secs_f32(0.0);
	let mut fps_counter = FPSCounter::new();

	let mut ctx = Ctx {
		window: &mut window,
		gfx: &mut gfx,
		time: time,
		quit: false,
		fps_counter: &mut fps_counter,
	};

	let mut s = S::init(&mut ctx)?;
	let mut last_frame_time = Instant::now();

	window.run(move |mut window, e| {

		let mut ctx = Ctx {
			window: &mut window,
			gfx: &mut gfx,
			time: time,
			fps_counter: &mut fps_counter,
			quit: false,
		};

		match e {

			window::WindowEvent::Input(ie) => {
				s.event(&mut ctx, &ie)?;
			},

			window::WindowEvent::Frame => {

				let dt = last_frame_time.elapsed();

				time += dt;
				ctx.fps_counter.tick(dt);
				last_frame_time = Instant::now();
				ctx.time = time;

				s.update(&mut ctx, dt)?;
				ctx.begin_frame();
				s.draw(&mut ctx)?;
				ctx.end_frame();

			},

		}

		return Ok(!ctx.quit);

	})?;

	return Ok(());

}

impl<'a> Ctx<'a> {

	pub fn time(&self) -> Duration {
		return self.time;
	}

	pub fn fps(&self) -> u16 {
		return self.fps_counter.fps();
	}

	pub fn quit(&mut self) {
		self.quit = true;
	}

}

impl<'a> Ctx<'a> {

	pub fn key_down(&self, k: Key) -> bool {
		return self.window.key_down(k);
	}

	pub fn key_mods(&self) -> input::KeyMod {
		return self.window.key_mods();
	}

	pub fn mouse_down(&self, m: Mouse) -> bool {
		return self.window.mouse_down(m);
	}

	pub fn width(&self) -> i32 {
		return self.window.width();
	}

	pub fn height(&self) -> i32 {
		return self.window.height();
	}

	pub fn dpi(&self) -> f32 {
		return self.window.dpi();
	}

	pub fn mouse_pos(&self) -> Vec2 {
		return self.window.mouse_pos();
	}

	pub fn set_mouse_pos(&mut self, p: Vec2) -> Result<()> {
		return self.window.set_mouse_pos(p);
	}

	pub fn clip_to_screen(&self, p: Vec2) -> Vec2 {
		return self.window.clip_to_screen(p);
	}

	pub fn screen_to_clip(&self, p: Vec2) -> Vec2 {
		return self.window.screen_to_clip(p);
	}

	pub fn set_fullscreen(&mut self, b: bool) {
		self.window.set_fullscreen(b);
	}

	pub fn is_fullscreen(&self) -> bool {
		return self.window.is_fullscreen();
	}

	pub fn toggle_fullscreen(&mut self) {
		self.window.toggle_fullscreen();
	}

	pub fn set_cursor_hidden(&mut self, b: bool) {
		self.window.set_cursor_hidden(b);
	}

	pub fn is_cursor_hidden(&self) -> bool {
		return self.window.is_cursor_hidden();
	}

	pub fn toggle_cursor_hidden(&mut self) {
		self.window.toggle_cursor_hidden();
	}

	pub fn set_cursor_locked(&mut self, b: bool) {
		self.window.set_cursor_locked(b);
	}
	pub fn is_cursor_locked(&self) -> bool {
		return self.window.is_cursor_locked();
	}

	pub fn toggle_cursor_locked(&mut self) {
		self.window.toggle_cursor_locked();
	}

	pub fn set_title(&mut self, s: &str) {
		self.window.set_title(s);
	}

	pub fn title(&self) -> &str {
		return self.window.title();
	}

}

