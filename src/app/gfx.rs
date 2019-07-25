// wengwengweng

use std::rc::Rc;

#[cfg(feature = "img")]
use crate::img::Image;
use crate::*;
use crate::math::*;
use super::gl;

pub struct Ctx {
	device: gl::Device,
	empty_tex: gl::Texture,
	_tex: gl::Texture,
	_vao: gl::VertexArray,
	_ibuf: gl::IndexBuffer,
	_program: gl::Program,
}

impl Ctx {

    pub(super) fn new(w: &window::Ctx, conf: &app::Conf) -> Self {

		let device = gl::Device::from_loader(|s| {
			w.windowed_ctx.get_proc_address(s) as *const _
		});

		let img = img::Image::from_bytes(include_bytes!("../res/CP437.png")).unwrap();
		let tex = gl::Texture::new(&device, img.width() as i32, img.height() as i32).unwrap();

		tex.data(&img.into_raw());

		let empty_tex = gl::Texture::new(&device, 1, 1).unwrap();

		empty_tex.data(&[255, 255, 255, 255]);

		let vbuf = gl::VertexBuffer::new(&device, 36, 9, gl::BufferUsage::Static).unwrap();
		let vao = gl::VertexArray::new(&device).unwrap();

		let model =
			Mat4::scale(vec3!(tex.width * 2, tex.height * 2, 1))
			* Mat4::rotate(15f32.to_radians(), Dir::Z)
			* Mat4::translate(vec3!(w.width() / 2, w.height() / 2, 0));

		let p1 = model.forward(vec4!(-0.5, 0.5, 0, 1));
		let p2 = model.forward(vec4!(0.5, 0.5, 0, 1));
		let p3 = model.forward(vec4!(0.5, -0.5, 0, 1));
		let p4 = model.forward(vec4!(-0.5, -0.5, 0, 1));

		vbuf.data(&[
			// pos       // colors        // uv
			p1.x, p1.y, p1.z,   1.0, 1.0, 1.0, 1.0,  0.0, 0.0,  // top left
			p2.x, p2.y, p2.z,   1.0, 1.0, 1.0, 1.0,   1.0, 0.0, // top right
			p3.x, p3.y, p3.z,   1.0, 1.0, 1.0, 1.0,   1.0, 1.0, // bottom right
			p4.x, p4.y, p4.z,   1.0, 1.0, 1.0, 1.0,   0.0, 1.0, // bottom left
		], 0);

		let ibuf = gl::IndexBuffer::new(&device, 6, gl::BufferUsage::Static).unwrap();

		ibuf.data(&[
			0, 1, 3,
			1, 2, 3,
		], 0);

		vao.attr(&vbuf, 0, 3, 0);
		vao.attr(&vbuf, 1, 4, 3);
		vao.attr(&vbuf, 2, 2, 7);

		let program = gl::Program::new(&device, include_str!("test.vert"), include_str!("test.frag")).expect("oh no");

		program.send("u_color", color!(0, 0, 1, 1));

		let proj = math::ortho(0.0, (w.width() as f32), 0.0, (w.height() as f32), -1.0, 1.0);
// 			* Mat4::translate(vec3!(w.width() / 2, w.height() / 2, 0))
// 			* Mat4::scale(vec3!(tex.width, tex.height, 1));

		program.send("u_proj", proj);

		let ctx = Self {
			device: device,
			empty_tex: empty_tex,
			_tex: tex,
			_vao: vao,
			_ibuf: ibuf,
			_program: program,
		};

		ctx.clear_color(conf.clear_color);
		ctx.clear();

		return ctx;

	}

	pub fn clear_color(&self, c: Color) {
		self.device.clear_color(c);
	}

	pub fn clear(&self) {
		self.device.clear();
		self.device.draw_elements(&self._vao, &self._ibuf, &self._program, &self._tex, 6);
	}

}

expose!(gfx, clear_color(c: Color));
expose!(gfx, clear());

#[derive(Clone)]
pub struct Texture {
	handle: Rc<gl::Texture>,
}

#[cfg(feature = "img")]
impl Texture {

	pub fn from_image(ctx: &app::Ctx, img: Image) -> Result<Self> {

		let w = img.width() as i32;
		let h = img.height() as i32;
		let handle = gl::Texture::new(&ctx.gfx.device, w, h)?;

		handle.data(&img.into_raw());

		return Ok(Self {
			handle: Rc::new(handle),
		});

	}

	pub fn from_file(ctx: &app::Ctx, fname: &str) -> Result<Self> {
		return Self::from_image(ctx, Image::from_file(fname)?);
	}

	pub fn from_bytes(ctx: &app::Ctx, data: &[u8]) -> Result<Self> {
		return Self::from_image(ctx, Image::from_bytes(data)?);
	}

	pub fn from_pixels(ctx: &app::Ctx, w: u32, h: u32, pixels: &[u8]) -> Result<Self> {
		return Self::from_image(ctx, Image::from_pixels(w, h, pixels));
	}

	pub fn width(&self) -> i32 {
		return self.handle.width;
	}

	pub fn height(&self) -> i32 {
		return self.handle.height;
	}

}

#[derive(Clone)]
pub struct Canvas {

	handle: Rc<gl::Framebuffer>,
// 	tex: Texture,
// 	width: u32,
// 	height: u32,

}

#[cfg(feature = "img")]
impl Canvas {

	pub fn new(ctx: &app::Ctx, width: i32, height: i32) -> Result<Self> {

		let handle = gl::Framebuffer::new(&ctx.gfx.device, width, height)?;
// 		let pixels = vec![0.0 as u8; (width * height * 4) as usize];
// 		let tex = Texture::from_pixels(width, height, &pixels);

// 		handle.attach(&*tex.handle);

		return Ok(Self {
			handle: Rc::new(handle),
// 			tex: tex,
// 			width: width,
// 			height: height,
		});

	}

}

