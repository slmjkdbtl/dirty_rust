// wengwengweng

use std::rc::Rc;

use crate::math::*;

use glow::Context;
type GLContext = glow::native::Context;

#[cfg(feature = "img")]
use crate::img::Image;

use crate::*;

pub struct Ctx {
	gl: Rc<GLContext>,
}

impl Ctx {

    pub fn new(w: &window::Ctx, conf: &app::Conf) -> Self {

		let gl = GLContext::from_loader_function(|s| {
			w.windowed_ctx.get_proc_address(s) as *const _
		});

		let ctx = Self {
			gl: Rc::new(gl),
		};

		ctx.clear_color(conf.clear_color);
		ctx.clear();

		return ctx;

	}

	pub fn clear_color(&self, c: Color) {
		unsafe {
			self.gl.clear_color(c.r, c.g, c.b, c.a);
		}
	}

	pub fn clear(&self) {
		unsafe {
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}

}

expose!(gfx, clear_color(c: Color));
expose!(gfx, clear());

#[derive(Clone)]
pub struct Texture {
// 	handle: Arc<ggl::Texture>,
}

#[cfg(feature = "img")]
impl Texture {

	pub fn from_image(img: Image) -> Self {

// 		let handle = ggl::Texture::new(img.width(), img.height());

// 		handle.data(&img.into_raw());

		return Self {
// 			handle: Arc::new(handle),
		};

	}

	pub fn from_file(fname: &str) -> Result<Self> {
		return Ok(Self::from_image(Image::from_file(fname)?));
	}

	pub fn from_bytes(data: &[u8]) -> Result<Self> {
		return Ok(Self::from_image(Image::from_bytes(data)?));
	}

	pub fn from_pixels(w: u32, h: u32, pixels: &[u8]) -> Self {
		return Self::from_image(Image::from_pixels(w, h, pixels));
	}

}

#[derive(Clone)]
pub struct Canvas {

// 	handle: Arc<ggl::Framebuffer>,
// 	tex: Texture,
// 	width: u32,
// 	height: u32,

}

#[cfg(feature = "img")]
impl Canvas {

	pub fn new(width: u32, height: u32) -> Self {

// 		let handle = ggl::Framebuffer::new();
// 		let pixels = vec![0.0 as u8; (width * height * 4) as usize];
// 		let tex = Texture::from_pixels(width, height, &pixels);

// 		handle.attach(&*tex.handle);

		return Self {
// 			handle: Arc::new(handle),
// 			tex: tex,
// 			width: width,
// 			height: height,
		};

	}

}

