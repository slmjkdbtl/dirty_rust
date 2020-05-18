// wengwengweng

use crate::*;
use window::WindowCtx;

pub use conf::Conf;

pub struct Ctx<'a> {

	#[cfg(not(web))]
	window: &'a mut native::Window,
	#[cfg(web)]
	window: &'a mut web::Window,

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

pub fn run<S: State>(conf: conf::Conf) -> Result<()> {

	#[cfg(not(web))]
	let window = native::Window::new(&conf)?;

	#[cfg(web)]
	let window = web::Window::new(&conf)?;

	let gl = window.gl();

	gl.enable(gl::Capability::Blend);
	gl.enable(gl::Capability::DepthTest);
	gl.blend_func(gl::BlendFac::SrcAlpha, gl::BlendFac::OneMinusSrcAlpha);
	gl.depth_func(gl::Cmp::LessOrEqual);
	gl.clear_color(0.0, 0.0, 0.0, 1.0);

	if conf.cull_face {
		gl.enable(gl::Capability::CullFace);
		gl.cull_face(gl::Face::Back);
		gl.front_face(gl::CullMode::CounterClockwise);
	}

	gl.clear(gl::Surface::Color);
	gl.clear(gl::Surface::Depth);
	gl.clear(gl::Surface::Stencil);
	window.swap()?;

	window.run(move |mut window, e| {

		let mut ctx = Ctx {
			window: &mut window,
		};

		match e {
			window::WindowEvent::Input(ie) => {
				// ...
			},
			window::WindowEvent::Frame => {
				// ...
			},
		}

		return Ok(());
	});

	return Ok(());

}

