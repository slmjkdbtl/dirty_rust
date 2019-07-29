// wengwengweng

use std::rc::Rc;
use std::collections::HashMap;

#[cfg(feature = "img")]
use crate::img::Image;

use crate::*;
use crate::math::*;
use super::*;

use gl::VertexLayout;
use gl::Shape;

#[derive(Clone, Default)]
pub(super) struct State {
	transform: Mat4,
	color: Color,
}

pub(super) struct QuadShape {
	transform: Mat4,
	quad: Quad,
	color: Color,
	radius: f32,
}

impl QuadShape {
	fn new(t: Mat4, q: Quad, c: Color, r: f32) -> Self {
		return Self {
			transform: t,
			quad: q,
			color: c,
			radius: r,
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
		let r = self.radius;
		let p1 = t.forward(vec4!(-0.5, 0.5, 0, 1));
		let p2 = t.forward(vec4!(0.5, 0.5, 0, 1));
		let p3 = t.forward(vec4!(0.5, -0.5, 0, 1));
		let p4 = t.forward(vec4!(-0.5, -0.5, 0, 1));

		Self::Vertex::new(vec2!(p1.x, p1.y), vec2!(q.x, q.y + q.h), *c, r).push(queue);
		Self::Vertex::new(vec2!(p2.x, p2.y), vec2!(q.x + q.w, q.y + q.h), *c, r).push(queue);
		Self::Vertex::new(vec2!(p3.x, p3.y), vec2!(q.x + q.w, q.y), *c, r).push(queue);
		Self::Vertex::new(vec2!(p4.x, p4.y), vec2!(q.x, q.y), *c, r).push(queue);

	}

	fn indices() -> Vec<u32> {
		return vec![0, 1, 3, 1, 2, 3];
	}

}

pub(super) struct Vertex2D {
	pos: Vec2,
	uv: Vec2,
	color: Color,
	radius: f32,
}

impl Vertex2D {
	fn new(pos: Vec2, uv: Vec2, color: Color, radius: f32) -> Self {
		return Self {
			pos: pos,
			uv: uv,
			color: color,
			radius: radius,
		};
	}
}

impl VertexLayout for Vertex2D {

	const STRIDE: usize = 9;

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
			self.radius,
		]);
	}

	fn attrs() -> Vec<gl::VertexAttr> {

		return vec![
			gl::VertexAttr::new("pos", 2, 0),
			gl::VertexAttr::new("uv", 2, 2),
			gl::VertexAttr::new("color", 4, 4),
			gl::VertexAttr::new("radius", 1, 8),
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
			Origin::BottomLeft => math::ortho(0.0, w, 0.0, h, -1.0, 1.0),
			Origin::TopRight => math::ortho(w, 0.0, h, 0.0, -1.0, 1.0),
			Origin::BottomRight => math::ortho(-w, 0.0, 0.0, h, -1.0, 1.0),
		};

	}

}

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
	fn color(&mut self, c: Color);

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

	if let Some(tex) = &ctx.cur_tex {

		tex.handle.bind();
		ctx.batched_renderer.flush(&ctx.gl, &ctx.cur_shader.handle);
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

	fn color(&mut self, c: Color) {
		self.state.color = c;
	}

	fn draw(&mut self, thing: impl Drawable) -> Result<()> {
		return thing.draw(self);
	}

	fn draw_on(&mut self, canvas: &Canvas, mut f: impl FnMut(&mut Ctx) -> Result<()>) -> Result<()> {

		canvas.handle.bind();
		f(self)?;
		canvas.handle.unbind();

		return Ok(());

	}

	fn draw_with(&mut self, shader: &Shader, mut f: impl FnMut(&mut Ctx) -> Result<()>) -> Result<()> {

		self.cur_shader = shader.clone();
		f(self)?;
		self.cur_shader = self.default_shader.clone();

		return Ok(());

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

	pub fn from_file(ctx: &Ctx, fname: &str) -> Result<Self> {
		return Self::from_image(ctx, Image::from_file(fname)?);
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

	tex: gfx::Texture,
	map: HashMap<char, Quad>,
	quad_size: Vec2,
	grid_size: Size,

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

		let size = Size::new(tw as u32 / cols as u32, th as u32 / rows as u32);

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
			grid_size: size,

		});

	}

	/// get current font width for string
	pub fn width(&self) -> u32 {
		return self.grid_size.w;
	}

	/// get current text height
	pub fn height(&self) -> u32 {
		return self.grid_size.h;
	}

}

#[derive(Clone, PartialEq)]
pub struct Shader {
	handle: Rc<gl::Program>,
}

impl Shader {

	pub(super) fn from_handle(handle: gl::Program) -> Self {
		return Self {
			handle: Rc::new(handle),
		};
	}

	pub fn new(ctx: &Ctx, vert: &str, frag: &str) -> Result<Self> {
		return Ok(Self::from_handle(gl::Program::new(&ctx.gl, vert, frag)?));
	}

	pub fn send<T: gl::UniformValue>(&self, name: &str, value: T) {
		self.handle.send(name, value);
	}

}

#[derive(Clone, PartialEq)]
pub struct Canvas {

	handle: Rc<gl::Framebuffer>,
	tex: Texture,

}

#[cfg(feature = "img")]
impl Canvas {

	pub fn new(ctx: &Ctx, width: u32, height: u32) -> Result<Self> {

		let pixels = vec![0.0 as u8; (width * height * 4) as usize];
		let tex = Texture::from_pixels(&ctx, width, height, &pixels)?;
		let handle = gl::Framebuffer::new(&ctx.gl, &tex.handle)?;

		return Ok(Self {
			handle: Rc::new(handle),
			tex: tex,
		});

	}

}

pub trait Drawable {
	fn draw(&self, ctx: &mut Ctx) -> Result<()>;
}

pub struct Sprite<'a> {
	tex: &'a gfx::Texture,
	quad: Quad,
	offset: Vec2,
	radius: f32,
}

impl<'a> Sprite<'a> {
	pub fn quad(mut self, quad: Quad) -> Self {
		self.quad = quad;
		return self;
	}
	pub fn offset(mut self, offset: Vec2) -> Self {
		self.offset = offset;
		return self;
	}
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = r;
		return self
	}
}

pub fn sprite<'a>(tex: &'a gfx::Texture) -> Sprite<'a> {
	return Sprite {
		tex: tex,
		quad: quad!(0, 0, 1, 1),
		offset: vec2!(0),
		radius: 0.0,
	};
}

impl<'a> Drawable for Sprite<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let wrapped_tex = Some(self.tex.clone());
		let scale = vec2!(self.tex.width(), self.tex.height()) * vec2!(self.quad.w, self.quad.h);

		if ctx.cur_tex != wrapped_tex {
			if ctx.cur_tex.is_some() {
				flush(ctx);
			}
			ctx.cur_tex = wrapped_tex;
		}

		ctx.push();
		ctx.scale(scale);
		ctx.translate(self.offset * -0.5);
		ctx.batched_renderer.push(gfx::QuadShape::new(ctx.state.transform, self.quad, ctx.state.color, 0.0))?;
		ctx.pop()?;

		return Ok(());

	}

}

pub struct Text<'a> {
	txt: &'a str,
	font: Option<&'a Font>,
	offset: Vec2,
}

impl<'a> Text<'a> {
	pub fn font(mut self, font: &'a Font) -> Self {
		self.font = Some(font);
		return self;
	}
	pub fn offset(mut self, offset: Vec2) -> Self {
		self.offset = offset;
		return self;
	}
}

pub fn text<'a>(txt: &'a str) -> Text<'a> {
	return Text {
		txt: txt,
		font: None,
		offset: vec2!(0),
	};
}

impl<'a> Drawable for Text<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let font;

		if let Some(f) = self.font {
			font = f.clone();
		} else {
			font = ctx.default_font.clone();
		}

		let len = self.txt.len();
		let gw = font.width();
		let gh = font.height();
		let tw = font.width() * len as u32;
		let th = gh;
		let w = font.quad_size.x * font.tex.width() as f32;
		let h = font.quad_size.y * font.tex.height() as f32;
		let tex = font.tex.clone();
		let offset = vec2!(gw as f32 * (len as f32 * -0.5 + 0.5), 0);
		let offset = offset + self.offset * vec2!(tw, th) * -0.5;

		ctx.push();
		ctx.translate(offset);

		for (i, ch) in self.txt.chars().enumerate() {

			let x = i as f32 * w;

			if ch != ' ' {

				if let Some(quad) = font.map.get(&ch) {
					ctx.draw(sprite(&tex).quad(*quad))?;
				}

			}

			ctx.translate(vec2!(w, 0));

		}

		ctx.pop()?;

		return Ok(());

	}

}

pub struct Line {
	p1: Vec2,
	p2: Vec2,
	width: f32,
}

impl Line {
	pub fn width(mut self, w: f32) -> Self {
		self.width = w;
		return self;
	}
}

pub fn line(p1: Vec2, p2: Vec2) -> Line {
	return Line {
		p1: p1,
		p2: p2,
		width: 1.0,
	};
}

impl Drawable for Line {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		let len = ((self.p2.x - self.p1.x).powi(2) + (self.p2.y - self.p1.y).powi(2)).sqrt();
		let rot = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);

		ctx.push();
		ctx.translate(self.p1);
		ctx.rotate(rot);
		ctx.draw(rect(len, self.width))?;
		ctx.pop()?;

		return Ok(());

	}

}

pub struct Rect {
	width: f32,
	height: f32,
	radius: f32,
	stroke: Option<f32>,
}

pub fn rect(w: f32, h: f32) -> Rect {
	return Rect {
		width: w,
		height: h,
		radius: 0.0,
		stroke: None,
	};
}

impl Rect {
	pub fn radius(mut self, r: f32) -> Self {
		self.radius = r;
		return self
	}
	pub fn stroke(mut self, s: f32) -> Self {
		self.stroke = Some(s);
		return self
	}
}

impl Drawable for Rect {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		ctx.push();
		ctx.scale(vec2!(self.width, self.height));
		ctx.draw(sprite(&ctx.empty_tex.clone()))?;
		ctx.pop()?;

		if let Some(stroke) = self.stroke {
			unimplemented!();
		}

		return Ok(());

	}

}

pub struct Points<'a> {
	pts: &'a[Vec2],
	size: f32,
}

impl<'a> Points<'a> {
	pub fn size(mut self, s: f32) -> Self {
		self.size = s;
		return self;
	}
}

pub fn pts<'a>(pts: &'a[Vec2]) -> Points<'a> {
	return Points {
		pts: pts,
		size: 1.0,
	};
}

impl<'a> Drawable for Points<'a> {

	fn draw(&self, ctx: &mut Ctx) -> Result<()> {

		for pt in self.pts {
			ctx.push();
			ctx.translate(*pt);
			ctx.draw(rect(self.size, self.size))?;
			ctx.pop()?;
		}

		return Ok(());

	}

}

