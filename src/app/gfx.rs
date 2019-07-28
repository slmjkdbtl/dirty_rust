// wengwengweng

use std::rc::Rc;
use std::collections::HashMap;

#[cfg(feature = "img")]
use crate::img::Image;
use crate::*;
use crate::math::*;
use super::gl;

use gl::VertexLayout;
use gl::Shape;

const MAX_DRAWS: usize = 65536;

const TEMPLATE_2D_VERT: &str = include_str!("../res/2d_template.vert");
const TEMPLATE_2D_FRAG: &str = include_str!("../res/2d_template.frag");

const DEFAULT_2D_VERT: &str = include_str!("../res/2d_default.vert");
const DEFAULT_2D_FRAG: &str = include_str!("../res/2d_default.frag");

const DEFAULT_FONT_IMG: &[u8] = include_bytes!("../res/CP437.png");
const DEFAULT_FONT_COLS: usize = 32;
const DEFAULT_FONT_ROWS: usize = 8;
const DEFAULT_FONT_CHARS: &str = r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##;

pub struct Ctx {
	pub(super) device: Rc<gl::Device>,
	batched_renderer: gl::BatchedRenderer<QuadShape>,
	cur_tex: Option<Texture>,
	default_shader: Shader,
	cur_shader: Shader,
	default_font: Font,
	cur_font: Font,
	draw_calls_last: usize,
	draw_calls: usize,
	state: State,
	state_stack: Vec<State>,
}

#[derive(Clone, Default)]
struct State {
	transform: Mat4,
	color: Color,
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

		Self::Vertex::new(vec2!(p1.x, p1.y), *c, vec2!(q.x, q.y + q.h)).push(queue);
		Self::Vertex::new(vec2!(p2.x, p2.y), *c, vec2!(q.x + q.w, q.y + q.h)).push(queue);
		Self::Vertex::new(vec2!(p3.x, p3.y), *c, vec2!(q.x + q.w, q.y)).push(queue);
		Self::Vertex::new(vec2!(p4.x, p4.y), *c, vec2!(q.x, q.y)).push(queue);

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
			Origin::Center => math::ortho(-w / 2.0, w / 2.0, h / 2.0, -h / 2.0, -1.0, 1.0),
			Origin::TopLeft => math::ortho(0.0, w, h, 0.0, -1.0, 1.0),
			Origin::BottomLeft => math::ortho(0.0, w, 0.0, h, -1.0, 1.0),
			Origin::TopRight => math::ortho(w, 0.0, h, 0.0, -1.0, 1.0),
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

		let vert_src = TEMPLATE_2D_VERT.replace("###REPLACE###", DEFAULT_2D_VERT);
		let frag_src = TEMPLATE_2D_FRAG.replace("###REPLACE###", DEFAULT_2D_FRAG);

		let shader = Shader::from_handle(gl::Program::new(&device, &vert_src, &frag_src)?);
		let proj = Origin::TopLeft.to_ortho(window.width(), window.height());

		shader.send("projection", proj);

		let font_img = img::Image::from_bytes(DEFAULT_FONT_IMG)?;
		let font_tex = gl::Texture::new(&device, font_img.width() as i32, font_img.height() as i32)?;
		font_tex.data(&font_img.into_raw());
		let font_tex = Texture::from_handle(font_tex);

		let font = Font::new(
			font_tex,
			DEFAULT_FONT_COLS,
			DEFAULT_FONT_ROWS,
			DEFAULT_FONT_CHARS,
		)?;

		let ctx = Self {
			device: device,
			cur_tex: None,
			batched_renderer: batched_renderer,
			default_shader: shader.clone(),
			cur_shader: shader,
			default_font: font.clone(),
			cur_font: font,
			draw_calls: 0,
			draw_calls_last: 0,
			state: State::default(),
			state_stack: Vec::with_capacity(16),
		};

		return Ok(ctx);

	}

	pub(super) fn begin(&mut self) {

		self.draw_calls_last = self.draw_calls;
		self.draw_calls = 0;
		self.clear();

	}

	pub(super) fn end(&mut self) {

		self.flush();
		self.state = State::default();
		self.state_stack.clear();

	}

	fn flush(&mut self) {

		if let Some(tex) = &self.cur_tex {

			tex.handle.bind();
			self.batched_renderer.flush(&self.device, &self.cur_shader.handle);
			tex.handle.unbind();
			self.draw_calls += 1;

		}

	}

	pub fn clear_color(&self, c: Color) {
		self.device.clear_color(c);
	}

	pub fn clear(&self) {
		self.device.clear();
	}

	pub fn draw_calls(&self) -> usize {
		return self.draw_calls_last;
	}

	pub fn push(&mut self) {
		self.state_stack.push(self.state.clone());
	}

	pub fn pop(&mut self) -> Result<()> {
		self.state = self.state_stack.pop().ok_or(Error::StateStack)?;
		return Ok(());
	}

	pub fn translate(&mut self, pos: Vec2) {
		self.state.transform = self.state.transform * Mat4::translate(vec3!(pos.x, pos.y, 0));
	}

	pub fn rotate(&mut self, angle: f32) {
		self.state.transform = self.state.transform * Mat4::rotate(angle, Dir::Z);
	}

	pub fn scale(&mut self, scale: Vec2) {
		self.state.transform = self.state.transform * Mat4::scale(vec3!(scale.x, scale.y, 1));
	}

	pub fn draw(&mut self, tex: &Texture, quad: Rect) -> Result<()> {

		let wrapped_tex = Some(tex.clone());
		let scale = vec2!(tex.width(), tex.height()) * vec2!(quad.w, quad.h);

		if self.cur_tex != wrapped_tex {
			if self.cur_tex.is_some() {
				self.flush();
			}
			self.cur_tex = wrapped_tex;
		}

		self.push();
		self.scale(scale);
		self.batched_renderer.push(QuadShape::new(self.state.transform, self.state.color, quad))?;
		self.pop()?;

		return Ok(());

	}

	/// draw text
	pub fn text(&mut self, s: &str) -> Result<()> {

		let w = self.cur_font.quad_size.x * self.cur_font.tex.width() as f32;
		let h = self.cur_font.quad_size.y * self.cur_font.tex.height() as f32;
		let tex = self.cur_font.tex.clone();

		self.push();

		for (i, ch) in s.chars().enumerate() {

			let x = i as f32 * w;

			if ch != ' ' {

				if let Some(quad) = self.cur_font.map.get(&ch) {
					self.draw(&tex, *quad)?;
				}

			}

			self.translate(vec2!(w, 0));

		}

		self.pop();

		return Ok(());

	}

}

expose!(gfx, clear_color(c: Color));
expose!(gfx, clear());
expose!(gfx, draw_calls() -> usize);
expose!(gfx(mut), draw(tex: &Texture, quad: Rect) -> Result<()>);
expose!(gfx(mut), text(txt: &str) -> Result<()>);
expose!(gfx(mut), push());
expose!(gfx(mut), pop() -> Result<()>);
expose!(gfx(mut), translate(pos: Vec2));
expose!(gfx(mut), rotate(angle: f32));
expose!(gfx(mut), scale(scale: Vec2));

#[derive(Clone, PartialEq)]
pub struct Texture {
	handle: Rc<gl::Texture>,
}

#[cfg(feature = "img")]
impl Texture {

	fn from_handle(handle: gl::Texture) -> Self {
		return Self {
			handle: Rc::new(handle),
		};
	}

	pub fn from_image(ctx: &app::Ctx, img: Image) -> Result<Self> {

		let w = img.width() as i32;
		let h = img.height() as i32;
		let handle = gl::Texture::new(&ctx.gfx.device, w, h)?;

		handle.data(&img.into_raw());

		return Ok(Self::from_handle(handle));

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

/// bitmap font
#[derive(Clone, PartialEq)]
pub struct Font {

	tex: gfx::Texture,
	map: HashMap<char, Rect>,
	quad_size: Vec2,
	grid_size: Size,

}

impl Font {

	/// creat a bitmap font from a texture, and grid of characters
	pub fn new(tex: gfx::Texture, cols: usize, rows: usize, chars: &str) -> Result<Self> {

		let mut map = HashMap::new();
		let quad_size = vec2!(1.0 / cols as f32, 1.0 / rows as f32);
		let tw = tex.width();
		let th = tex.height();

		if (tw % cols as i32 != 0 || th % rows as i32 != 0) {
			return Err(Error::Font);
		}

		let size = Size::new(tw as u32 / cols as u32, th as u32 / rows as u32);

		for (i, ch) in chars.chars().enumerate() {

			map.insert(ch, rect!(

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

	fn from_handle(handle: gl::Program) -> Self {
		return Self {
			handle: Rc::new(handle),
		};
	}

	pub fn new(ctx: &app::Ctx, vert: &str, frag: &str) -> Result<Self> {
		return Ok(Self::from_handle(gl::Program::new(&ctx.gfx.device, vert, frag)?));
	}

	pub fn send<T: gl::UniformValue>(&self, name: &str, value: T) {
		self.handle.send(name, value);
	}

}

#[derive(Clone, PartialEq)]
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

