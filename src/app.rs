// wengwengweng

use std::time::Duration;

use instant::Instant;

use crate::*;
use window::WindowCtx;
pub use conf::*;

pub struct Ctx<'a> {

	#[cfg(not(web))]
	pub(crate) window: &'a mut native::Window,
	#[cfg(web)]
	pub(crate) window: &'a mut web::Window,

	pub(crate) gfx: &'a mut gfx::GfxCtx,

}

pub trait State: 'static + Sized {

	fn init(_: &mut Ctx) -> Result<Self>;

	fn event(&mut self, _: &mut Ctx, _: &input::Event) -> Result<()> {
		return Ok(());
	}

	fn update(&mut self, _: &mut Ctx) -> Result<()> {
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

	let mut ctx = Ctx {
		window: &mut window,
		gfx: &mut gfx,
	};

	let mut s = S::init(&mut ctx)?;
	let mut last_frame_time = Instant::now();
	let mut time = Duration::from_secs_f32(0.0);

	window.run(move |mut window, e| {

		let mut ctx = Ctx {
			window: &mut window,
			gfx: &mut gfx,
		};

		match e {

			window::WindowEvent::Input(ie) => {
				s.event(&mut ctx, &ie)?;
			},

			window::WindowEvent::Frame => {

				let dt = last_frame_time.elapsed();

				time += dt;
				last_frame_time = Instant::now();

				s.update(&mut ctx)?;
				ctx.begin_frame();
				s.draw(&mut ctx)?;
				ctx.end_frame();

			},
		}

		return Ok(());

	})?;

	return Ok(());

}

