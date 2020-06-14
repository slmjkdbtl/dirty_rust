// wengwengweng

use crate::*;
use window::*;
use conf::*;

/// The Main Trait
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

	fn quit(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

}

impl Launcher {
	pub fn run<S: State>(self) -> Result<()> {
		return run_with_conf::<S>(self.conf);
	}
}

/// run with configs, see methods under [Launcher](conf::Launcher)
pub fn launcher() -> Launcher {
	return Launcher::default();
}

/// run simple
pub fn run<S: State>() -> Result<()> {
	return launcher().run::<S>();
}

fn run_with_conf<S: State>(conf: conf::Conf) -> Result<()> {

	let mut window = window::Window::new(&conf)?;
	let mut gfx = gfx::Gfx::new(&window, &conf)?;
	let mut app = app::App::new(&conf);
	let mut audio = audio::Audio::new(&conf)?;

	window.swap()?;

	let mut ctx = Ctx {
		window: &mut window,
		gfx: &mut gfx,
		app: &mut app,
		audio: &mut audio,
	};

	let mut s = S::init(&mut ctx)?;

	window.run(move |mut window, e| {

		let mut ctx = Ctx {
			window: &mut window,
			gfx: &mut gfx,
			app: &mut app,
			audio: &mut audio,
		};

		match e {

			WindowEvent::Resize(w, h) => {
				ctx.gfx.resize(w, h);
			},

			WindowEvent::DPIChange(dpi) => {
				ctx.gfx.set_dpi(dpi);
			},

			WindowEvent::Input(ie) => {
				s.event(&mut ctx, &ie)?;
			},

			WindowEvent::Frame => {
				ctx.app.tick();
				s.update(&mut ctx)?;
				ctx.gfx.begin_frame();
				s.draw(&mut ctx)?;
				ctx.gfx.end_frame();
			},

			WindowEvent::Quit => {
				s.quit(&mut ctx)?;
			},

		}

		return Ok(());

	})?;

	return Ok(());

}

