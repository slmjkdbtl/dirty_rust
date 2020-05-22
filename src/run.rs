// wengwengweng

use crate::*;
use window::*;
use conf::*;

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
	let mut app = app::App::new();
	let mut audio = audio::Audio::new()?;

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

		}

		return Ok(());

	})?;

	return Ok(());

}

