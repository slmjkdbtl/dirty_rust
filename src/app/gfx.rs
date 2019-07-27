// wengwengweng

use std::rc::Rc;

#[cfg(feature = "img")]
use crate::img::Image;
use crate::*;
use crate::math::*;
use super::gl;

use gl::VertexLayout;
use gl::Shape;

const MAX_DRAWS: usize = 65536;

pub struct Ctx {
	pub(super) device: Rc<gl::Device>,
	cur_tex: Option<Texture>,
	batched_renderer: gl::BatchedRenderer<QuadShape>,
	program: gl::Program,
	draw_calls_last: usize,
	draw_calls: usize,
}

struct QuadShape {
	transform: Mat4,
	color: Color,
	quad: Rect,
}

impl QuadShape {
	fn new(t: Mat4, c: Color, q: Rect) -> Self {
		return Self {
			transform: t,
			color: c,
			quad: q,
		};
	}
}

impl Shape for QuadShape {

	type Vertex = Vertex2D;
	const COUNT: usize = 4;

	fn push(&self, queue: &mut Vec<f32>) {

		let t = &self.transform;
		let q = &self.quad;
		let c = &self.color;
		let p1 = t.forward(vec4!(-0.5, 0.5, 0, 1));
		let p2 = t.forward(vec4!(0.5, 0.5, 0, 1));
		let p3 = t.forward(vec4!(0.5, -0.5, 0, 1));
		let p4 = t.forward(vec4!(-0.5, -0.5, 0, 1));

		Self::Vertex::new(vec2!(p1.x, p1.y), *c, vec2!(q.x, q.y)).push(queue);
		Self::Vertex::new(vec2!(p2.x, p2.y), *c, vec2!(q.x + q.w, q.y)).push(queue);
		Self::Vertex::new(vec2!(p3.x, p3.y), *c, vec2!(q.x + q.w, q.y + q.h)).push(queue);
		Self::Vertex::new(vec2!(p4.x, p4.y), *c, vec2!(q.x, q.y + q.h)).push(queue);

	}

	fn indices() -> Vec<u32> {
		return vec![0, 1, 3, 1, 2, 3];
	}

}

struct Vertex2D {
	pos: Vec2,
	color: Color,
	uv: Vec2,
}

impl Vertex2D {
	fn new(pos: Vec2, color: Color, uv: Vec2) -> Self {
		return Self {
			pos: pos,
			color: color,
			uv: uv,
		};
	}
}

impl VertexLayout for Vertex2D {

	const STRIDE: usize = 8;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.color.r,
			self.color.g,
			self.color.b,
			self.color.a,
			self.uv.x,
			self.uv.y,
		]);
	}

	fn attrs() -> Vec<gl::VertexAttr> {

		return vec![
			gl::VertexAttr::new("pos", 2, 0),
			gl::VertexAttr::new("color", 4, 2),
			gl::VertexAttr::new("uv", 2, 6),
		];

	}
}

pub enum Origin {
	Center,
	TopLeft,
	BottomLeft,
	TopRight,
	BottomRight,
}

impl Origin {

	pub fn to_ortho(&self, w: i32, h: i32) -> Mat4 {

		let w = w as f32;
		let h = h as f32;

		return match self {
			Origin::Center => math::ortho(-w / 2.0, w / 2.0, -h / 2.0, h / 2.0, -1.0, 1.0),
			Origin::TopLeft => math::ortho(0.0, w, -h, 0.0, -1.0, 1.0),
			Origin::BottomLeft => math::ortho(0.0, w, 0.0, h, -1.0, 1.0),
			Origin::TopRight => math::ortho(-w, 0.0, -h, 0.0, -1.0, 1.0),
			Origin::BottomRight => math::ortho(-w, 0.0, 0.0, h, -1.0, 1.0),
		};

	}

}

impl Ctx {

    pub(super) fn new(window: &window::Ctx, conf: &app::Conf) -> Result<Self> {

		let device = gl::Device::from_loader(|s| {
			window.windowed_ctx.get_proc_address(s) as *const _
		});

		let device = Rc::new(device);

		device.enable(gl::Capability::Blend);
		device.blend_func_sep(gl::BlendFunc::SrcAlpha, gl::BlendFunc::OneMinusSrcAlpha, gl::BlendFunc::One, gl::BlendFunc::OneMinusSrcAlpha);
		device.clear_color(conf.clear_color);
		device.clear();
		window.swap()?;

		let batched_renderer = gl::BatchedRenderer::<QuadShape>::new(&device, MAX_DRAWS)?;

		let vert_src = include_str!("../res/2d_template.vert").replace("###REPLACE###", include_str!("../res/2d_default.vert"));
		let frag_src = include_str!("../res/2d_template.frag").replace("###REPLACE###", include_str!("../res/2d_default.frag"));

		let program = gl::Program::new(&device, &vert_src, &frag_src)?;
		let proj = Origin::Center.to_ortho(window.width(), window.height());

		program.send("projection", proj);

		let ctx = Self {
			device: device,
			cur_tex: None,
			batched_renderer: batched_renderer,
			program: program,
			draw_calls: 0,
			draw_calls_last: 0,
		};

		return Ok(ctx);

	}

	pub fn clear_color(&self, c: Color) {
		self.device.clear_color(c);
	}

	pub fn clear(&self) {
		self.device.clear();
	}

	pub fn begin(&mut self) {
		self.draw_calls_last = self.draw_calls;
		self.draw_calls = 0;
		self.clear();
	}

	pub fn end(&mut self) {
		self.flush();
	}

	pub fn flush(&mut self) {

		if let Some(tex) = &self.cur_tex {

			tex.handle.bind();
			self.batched_renderer.flush(&self.device, &self.program);
			tex.handle.unbind();
			self.draw_calls += 1;

		}

	}

	pub fn draw_calls(&self) -> usize {
		return self.draw_calls_last;
	}

	pub fn draw(&mut self, tex: &Texture, pos: Vec2, rot: f32, scale: Vec2, quad: Rect, c: Color) -> Result<()> {

		let scale = scale * vec2!(tex.width(), tex.height()) * vec2!(quad.w, quad.h);

		let model =
			Mat4::translate(vec3!(pos.x, pos.y, 0))
			* Mat4::rotate(rot, Dir::Z)
			* Mat4::scale(vec3!(scale.x, scale.y, 1));

		let wrapped_tex = Some(tex.clone());

		if self.cur_tex != wrapped_tex {
			if self.cur_tex.is_some() {
				self.flush();
			}
			self.cur_tex = wrapped_tex;
		}

		self.batched_renderer.push(QuadShape::new(model, c, quad))?;

		return Ok(());

	}

}

expose!(gfx, clear_color(c: Color));
expose!(gfx, clear());
expose!(gfx(mut), draw(tex: &Texture, pos: Vec2, rot: f32, scale: Vec2, quad: Rect, c: Color) -> Result<()>);
expose!(gfx, draw_calls() -> usize);

#[derive(Clone, PartialEq)]
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

