// wengwengweng

use std::mem;
use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;
use std::io::Cursor;

#[cfg(feature = "img")]
use crate::img::Image;

use crate::*;
use crate::math::*;
use super::*;

use gl::VertexLayout;
use gl::Shape;

pub use gl::UniformValue;
pub use gl::UniformType;

pub trait Gfx {

	fn clear_color(&self, c: Color);
	fn clear(&self);
	fn draw_calls(&self) -> usize;
	fn draw(&mut self, t: impl Drawable) -> Result<()>;
	fn draw_on(&mut self, canvas: &Canvas, f: impl FnMut(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_with(&mut self, shader: &Shader, f: impl FnMut(&mut Self) -> Result<()>) -> Result<()>;
	fn push(&mut self);
	fn pop(&mut self) -> Result<()>;
	fn translate(&mut self, pos: Vec2);
	fn rotate(&mut self, angle: f32);
	fn scale(&mut self, scale: Vec2);
	fn translate3d(&mut self, pos: Vec3);
	fn rotate_x(&mut self, angle: f32);
	fn rotate_y(&mut self, angle: f32);
	fn rotate_z(&mut self, angle: f32);
	fn scale3d(&mut self, scale: Vec3);
	fn color(&mut self, c: Color);
	fn reset(&mut self);

}

#[derive(Clone, Default)]
pub(super) struct State {
	pub transform: Mat4,
	pub color: Color,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Flip {
	None,
	X,
	Y,
	XY,
}

pub(super) struct QuadShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub origin: Origin,
	pub flip: Flip,
}

impl QuadShape {
	pub fn new(t: Mat4, q: Quad, c: Color, o: Origin, f: Flip) -> Self {
		return Self {
			transform: t,
			quad: q,
			color: c,
			origin: o,
			flip: f,
		};
	}
}

impl Shape for QuadShape {

	type Vertex = Vertex2D;
	const COUNT: usize = 4;

	fn push(&self, queue: &mut Vec<f32>) {

		let t = self.transform;
		let q = self.quad;
		let c = self.color;
		let offset = self.origin.as_pt() * 0.5;
		let p1 = t * (vec2!(-0.5, 0.5) - offset);
		let p2 = t * (vec2!(0.5, 0.5) - offset);
		let p3 = t * (vec2!(0.5, -0.5) - offset);
		let p4 = t * (vec2!(-0.5, -0.5) - offset);

		let mut u1 = vec2!(q.x, q.y + q.h);
		let mut u2 = vec2!(q.x + q.w, q.y + q.h);
		let mut u3 = vec2!(q.x + q.w, q.y);
		let mut u4 = vec2!(q.x, q.y);

		match self.flip {
			Flip::X => {
				mem::swap(&mut u1, &mut u2);
				mem::swap(&mut u4, &mut u3);
			},
			Flip::Y => {
				mem::swap(&mut u2, &mut u3);
				mem::swap(&mut u1, &mut u4);
			},
			Flip::XY => {
				mem::swap(&mut u2, &mut u4);
				mem::swap(&mut u1, &mut u3);
			},
			_ => {},
		}

		Self::Vertex::new(p1, u1, c).push(queue);
		Self::Vertex::new(p2, u2, c).push(queue);
		Self::Vertex::new(p3, u3, c).push(queue);
		Self::Vertex::new(p4, u4, c).push(queue);

	}

	fn indices() -> Vec<u32> {
		return vec![0, 1, 3, 1, 2, 3];
	}

}

pub(super) struct Vertex2D {
	pos: Vec2,
	uv: Vec2,
	color: Color,
}

impl Vertex2D {
	fn new(pos: Vec2, uv: Vec2, color: Color) -> Self {
		return Self {
			pos: pos,
			uv: uv,
			color: color,
		};
	}
}

impl VertexLayout for Vertex2D {

	const STRIDE: usize = 8;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.uv.x,
			self.uv.y,
			self.color.r,
			self.color.g,
			self.color.b,
			self.color.a,
		]);
	}

	fn attrs() -> Vec<gl::VertexAttr> {

		return vec![
			gl::VertexAttr::new("pos", 2, 0),
			gl::VertexAttr::new("uv", 2, 2),
			gl::VertexAttr::new("color", 4, 4),
		];

	}
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Origin {
	Center,
	TopLeft,
	BottomLeft,
	TopRight,
	BottomRight,
}

impl Origin {

	pub fn to_ortho(&self, w: u32, h: u32) -> Mat4 {

		let w = w as f32;
		let h = h as f32;

		return match self {
			Origin::Center => math::ortho(-w / 2.0, w / 2.0, h / 2.0, -h / 2.0, -1.0, 1.0),
			Origin::TopLeft => math::ortho(0.0, w, h, 0.0, -1.0, 1.0),
			Origin::BottomLeft => math::ortho(0.0, w, 0.0, -h, -1.0, 1.0),
			Origin::TopRight => math::ortho(-w, 0.0, h, 0.0, -1.0, 1.0),
			Origin::BottomRight => math::ortho(-w, 0.0, 0.0, -h, -1.0, 1.0),
		};

	}

	pub fn as_pt(&self) -> Vec2 {
		return match self {
			Origin::Center => vec2!(0, 0),
			Origin::TopLeft => vec2!(-1, -1),
			Origin::BottomLeft => vec2!(-1, 1),
			Origin::TopRight => vec2!(1, -1),
			Origin::BottomRight => vec2!(1, 1),
		};
	}

}

pub(super) fn origin(ctx: &app::Ctx) -> Origin {
	return ctx.origin;
}

pub(super) fn begin(ctx: &mut Ctx) {

	ctx.draw_calls_last = ctx.draw_calls;
	ctx.draw_calls = 0;
	ctx.clear();

}

pub(super) fn end(ctx: &mut Ctx) {

	flush(ctx);
	ctx.state = State::default();
	ctx.state_stack.clear();

}

pub(super) fn flush(ctx: &mut Ctx) {

	if ctx.batched_renderer.empty() {
		return;
	}

	if let Some(tex) = &ctx.cur_tex {

		tex.handle.bind();
		ctx.batched_renderer.flush(&ctx.gl, &ctx.cur_shader_2d.handle);
		tex.handle.unbind();
		ctx.draw_calls += 1;

	}

}

impl Gfx for Ctx {

	fn clear_color(&self, c: Color) {
		self.gl.clear_color(c);
	}

	fn clear(&self) {
		self.gl.clear();
	}

	fn draw_calls(&self) -> usize {
		return self.draw_calls_last;
	}

	fn push(&mut self) {
		self.state_stack.push(self.state.clone());
	}

	fn pop(&mut self) -> Result<()> {
		self.state = self.state_stack.pop().ok_or(Error::StateStack)?;
		return Ok(());
	}

	fn translate(&mut self, pos: Vec2) {
		self.state.transform = self.state.transform * Mat4::translate(vec3!(pos.x, pos.y, 0));
	}

	fn rotate(&mut self, angle: f32) {
		self.state.transform = self.state.transform * Mat4::rotate(angle, Dir::Z);
	}

	fn scale(&mut self, scale: Vec2) {
		self.state.transform = self.state.transform * Mat4::scale(vec3!(scale.x, scale.y, 1));
	}

	fn translate3d(&mut self, pos: Vec3) {
		self.state.transform = self.state.transform * Mat4::translate(pos);
	}

	fn rotate_x(&mut self, angle: f32) {
		self.state.transform = self.state.transform * Mat4::rotate(angle, Dir::X);
	}

	fn rotate_y(&mut self, angle: f32) {
		self.state.transform = self.state.transform * Mat4::rotate(angle, Dir::Y);
	}

	fn rotate_z(&mut self, angle: f32) {
		self.state.transform = self.state.transform * Mat4::rotate(angle, Dir::Z);
	}

	fn scale3d(&mut self, scale: Vec3) {
		self.state.transform = self.state.transform * Mat4::scale(scale);
	}

	fn color(&mut self, c: Color) {
		self.state.color = c;
	}

	fn draw(&mut self, thing: impl Drawable) -> Result<()> {
		return thing.draw(self);
	}

	fn draw_on(&mut self, canvas: &Canvas, mut f: impl FnMut(&mut Ctx) -> Result<()>) -> Result<()> {

		let mut flipped_proj = self.proj_2d.clone();

		if let Some(val) = flipped_proj.get_mut(1, 1) {
			*val = -*val;
		}

		if let Some(val) = flipped_proj.get_mut(3, 1) {
			*val = -*val;
		}

		flush(self);
		canvas.handle.bind();
		self.cur_shader_2d.send("proj", flipped_proj);
		self.push();
		self.reset();
		f(self)?;
		self.pop()?;
		flush(self);
		self.cur_shader_2d.send("proj", self.proj_2d);

		canvas.handle.unbind();

		return Ok(());

	}

	// TODO: user shader black screen
	fn draw_with(&mut self, shader: &Shader, mut f: impl FnMut(&mut Ctx) -> Result<()>) -> Result<()> {

		self.cur_shader_2d = shader.clone();
		f(self)?;
		flush(self);
		self.cur_shader_2d = self.default_shader_2d.clone();

		return Ok(());

	}

	fn reset(&mut self) {
		self.state = State::default();
	}

}

#[derive(Clone, PartialEq)]
pub struct Texture {
	handle: Rc<gl::Texture>,
}

#[cfg(feature = "img")]
impl Texture {

	pub(super) fn from_handle(handle: gl::Texture) -> Self {
		return Self {
			handle: Rc::new(handle),
		};
	}

	pub fn from_image(ctx: &Ctx, img: Image) -> Result<Self> {

		let w = img.width() as i32;
		let h = img.height() as i32;
		let handle = gl::Texture::new(&ctx.gl, w, h)?;

		handle.data(&img.into_raw());

		return Ok(Self::from_handle(handle));

	}

	pub fn from_file(ctx: &Ctx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_image(ctx, Image::from_file(path)?);
	}

	pub fn from_bytes(ctx: &Ctx, data: &[u8]) -> Result<Self> {
		return Self::from_image(ctx, Image::from_bytes(data)?);
	}

	pub fn from_pixels(ctx: &Ctx, w: u32, h: u32, pixels: &[u8]) -> Result<Self> {

		let handle = gl::Texture::new(&ctx.gl, w as i32, h as i32)?;
		handle.data(&pixels);
		return Ok(Self::from_handle(handle));

	}

	pub fn width(&self) -> i32 {
		return self.handle.width;
	}

	pub fn height(&self) -> i32 {
		return self.handle.height;
	}

}

/// bitmap font
#[derive(Clone, PartialEq)]
pub struct Font {

	pub(super) tex: gfx::Texture,
	pub(super) map: HashMap<char, Quad>,
	pub(super) quad_size: Vec2,
	grid_width: u32,
	grid_height: u32,

}

impl Font {

	/// creat a bitmap font from a texture, and grid of characters
	pub fn from_tex(tex: gfx::Texture, cols: usize, rows: usize, chars: &str) -> Result<Self> {

		let mut map = HashMap::new();
		let quad_size = vec2!(1.0 / cols as f32, 1.0 / rows as f32);
		let tw = tex.width();
		let th = tex.height();

		if (tw % cols as i32 != 0 || th % rows as i32 != 0) {
			return Err(Error::Font);
		}

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, quad!(

				(i % cols) as f32 * quad_size.x,
				(i / cols) as f32 * quad_size.y,
				quad_size.x,
				quad_size.y

			));

		}

		return Ok(Self {

			tex: tex,
			map: map,
			quad_size: quad_size,
			grid_width: tw as u32 / cols as u32,
			grid_height: th as u32 / rows as u32,

		});

	}

	/// get current font width for string
	pub fn width(&self) -> u32 {
		return self.grid_width;
	}

	/// get current text height
	pub fn height(&self) -> u32 {
		return self.grid_height;
	}

}

#[derive(Clone, PartialEq)]
pub struct Shader {
	pub(super) handle: Rc<gl::Program>,
}

impl Shader {

	pub(super) fn from_handle(handle: gl::Program) -> Self {
		return Self {
			handle: Rc::new(handle),
		};
	}

	pub fn effect(ctx: &Ctx, frag: &str) -> Result<Self> {

		let vert_src = TEMPLATE_2D_VERT.replace("###REPLACE###", DEFAULT_2D_VERT);
		let frag_src = TEMPLATE_2D_FRAG.replace("###REPLACE###", frag);

		return Self::from_code(ctx, &vert_src, &frag_src);

	}

	pub fn from_code(ctx: &Ctx, vert: &str, frag: &str) -> Result<Self> {
		return Ok(Self::from_handle(gl::Program::new(&ctx.gl, vert, frag)?));
	}

	pub fn send<T: gl::UniformValue>(&self, name: &str, value: T) {
		self.handle.send(name, value);
	}

}

#[derive(Clone, PartialEq)]
pub struct Canvas {

	handle: Rc<gl::Framebuffer>,
	pub(super) tex: Texture,

}

#[cfg(feature = "img")]
impl Canvas {

	pub fn new(ctx: &Ctx, width: u32, height: u32) -> Result<Self> {

		let dpi = ctx.dpi();
		let tw = (width as f64 * dpi) as u32;
		let th = (height as f64 * dpi) as u32;
		let pixels = vec![0.0 as u8; (tw * th * 4) as usize];
		let tex = Texture::from_pixels(&ctx, tw, th, &pixels)?;
		let handle = gl::Framebuffer::new(&ctx.gl, &tex.handle)?;

		return Ok(Self {
			handle: Rc::new(handle),
			tex: tex,
		});

	}

	pub fn width(&self) -> i32 {
		return self.tex.width();
	}

	pub fn height(&self) -> i32 {
		return self.tex.height();
	}

	pub fn capture(&self, path: impl AsRef<Path>) -> Result<()> {

		let tex = &self.tex;
		let buffer = tex.handle.get_data();

		image::save_buffer(
			path,
			&buffer,
			tex.width() as u32,
			tex.height() as u32,
			image::ColorType::RGBA(8),
		)?;

		return Ok(());

	}

}

pub struct Vertex3D {
	pos: Vec3,
}

impl VertexLayout for Vertex3D {

	const STRIDE: usize = 3;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.pos.z,
		]);
	}

	fn attrs() -> Vec<gl::VertexAttr> {

		return vec![
			gl::VertexAttr::new("pos", 3, 0),
		];

	}

}

pub struct Model {
	pub(super) vbuf: gl::VertexBuffer<Vertex3D>,
	pub(super) ibuf: gl::IndexBuffer,
	pub(super) len: usize,
}

impl Model {

	fn from_tobj(ctx: &Ctx, tobj: tobj::LoadResult) -> Result<Self> {

		let (models, mtls) = tobj?;
		let mesh = &models.get(0).ok_or(Error::ObjLoad)?.mesh;

		let vbuf = gl::VertexBuffer::<Vertex3D>::new(&ctx.gl, mesh.positions.len() / Vertex3D::STRIDE, gl::BufferUsage::Static)?;
		let ibuf = gl::IndexBuffer::new(&ctx.gl, mesh.indices.len(), gl::BufferUsage::Static)?;

		vbuf.data(0, &mesh.positions);
		ibuf.data(0, &mesh.indices);

		return Ok(Self {
			vbuf: vbuf,
			ibuf: ibuf,
			len: mesh.indices.len(),
		});

	}

	pub fn from_obj(ctx: &Ctx, obj: &str) -> Result<Self> {
		return Self::from_tobj(ctx, tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return Err(tobj::LoadError::GenericFailure);
		}));
	}

	pub fn from_obj_with_mtl(ctx: &Ctx, obj: &str, mtl: &str) -> Result<Self> {
		return Self::from_tobj(ctx, tobj::load_obj_buf(&mut Cursor::new(obj), |_| {
			return tobj::load_mtl_buf(&mut Cursor::new(mtl));
		}));
	}

	pub fn from_obj_file(ctx: &Ctx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_tobj(ctx, tobj::load_obj(path.as_ref()));
	}

}

pub trait Drawable {
	fn draw(&self, ctx: &mut Ctx) -> Result<()>;
}

