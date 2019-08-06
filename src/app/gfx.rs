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

	// clearing
	fn clear_color(&self, c: Color);
	fn clear(&self);

	// stats
	fn draw_calls(&self) -> usize;

	// drawing
	fn draw(&mut self, t: impl DrawCmd) -> Result<()>;
	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_with(&mut self, shader: &Shader, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// transform
	fn push(&mut self);
	fn pop(&mut self) -> Result<()>;
	fn translate(&mut self, pos: Vec2);
	fn rotate(&mut self, angle: f32);
	fn scale(&mut self, scale: Vec2);
	fn translate3d(&mut self, pos: Vec3);
	fn rotate3d(&mut self, angle: f32, axis: Vec3);
	fn scale3d(&mut self, scale: Vec3);
	fn matrix(&self) -> Mat4;
	fn apply(&mut self, m: Mat4);
	fn reset(&mut self);

	// camera
	fn cam_look(&mut self, yaw: f32, pitch: f32);
	fn cam_pos(&mut self, pos: Vec3);
	fn cam_front(&self) -> Vec3;

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

	fn attrs() -> gl::VertexAttrGroup {

		return gl::VertexAttrGroup::build()
			.add("pos", 2)
			.add("uv", 2)
			.add("color", 4)
			;

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
	ctx.transform = Mat4::identity();
	ctx.transform_stack.clear();

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
		self.transform_stack.push(self.transform.clone());
	}

	fn pop(&mut self) -> Result<()> {
		self.transform = self.transform_stack.pop().ok_or(Error::GfxPop)?;
		return Ok(());
	}

	fn translate(&mut self, pos: Vec2) {
		self.transform *= Mat4::translate(vec3!(pos.x, pos.y, 0));
	}

	fn rotate(&mut self, angle: f32) {
		self.transform *= Mat4::rotate(angle, vec3!(0, 0, 1));
	}

	fn scale(&mut self, scale: Vec2) {
		self.transform *= Mat4::scale(vec3!(scale.x, scale.y, 1));
	}

	fn translate3d(&mut self, pos: Vec3) {
		self.transform *= Mat4::translate(pos);
	}

	fn rotate3d(&mut self, angle: f32, axis: Vec3) {
		self.transform *= Mat4::rotate(angle, axis);
	}

	fn scale3d(&mut self, scale: Vec3) {
		self.transform *= Mat4::scale(scale);
	}

	fn matrix(&self) -> Mat4 {
		return self.transform;
	}

	fn apply(&mut self, m: Mat4) {
		self.transform = m;
	}

	fn reset(&mut self) {
		self.transform = Mat4::identity();
	}

	fn draw(&mut self, thing: impl DrawCmd) -> Result<()> {
		return thing.draw(self);
	}

	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Ctx) -> Result<()>) -> Result<()> {

		let mut flipped_proj = self.proj_2d.clone();

		if let Some(val) = flipped_proj.get_mut(1, 1) {
			*val = -*val;
		}

		if let Some(val) = flipped_proj.get_mut(3, 1) {
			*val = -*val;
		}

		flush(self);
		canvas.handle.bind();
		// TODO: fixed fullscreen framebuffer weirdness, but now weird resize
		self.gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
		self.cur_shader_2d.send("proj", flipped_proj);
		self.push();
		self.reset();
		f(self)?;
		self.pop()?;
		flush(self);
		self.cur_shader_2d.send("proj", self.proj_2d);

		canvas.handle.unbind();
		self.gl.viewport(0, 0, self.width() as i32 * self.dpi() as i32, self.height() as i32 * self.dpi() as i32);

		return Ok(());

	}

	fn draw_with(&mut self, shader: &Shader, f: impl FnOnce(&mut Ctx) -> Result<()>) -> Result<()> {

		flush(self);
		self.cur_shader_2d = shader.clone();
		self.cur_shader_2d.send("proj", self.proj_2d);
		f(self)?;
		flush(self);
		self.cur_shader_2d = self.default_shader_2d.clone();

		return Ok(());

	}

	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		flush(self);
		self.gl.clear_stencil();
		self.gl.enable(gl::Capability::StencilTest);
		self.gl.stencil_func(gl::StencilFunc::Never, 1, 0xff);
		self.gl.stencil_op(gl::StencilOp::Replace, gl::StencilOp::Replace, gl::StencilOp::Replace);
		mask(self)?;
		flush(self);
		self.gl.stencil_func(gl::StencilFunc::Equal, 1, 0xff);
		self.gl.stencil_op(gl::StencilOp::Keep, gl::StencilOp::Keep, gl::StencilOp::Keep);
		draw(self)?;
		flush(self);
		self.gl.disable(gl::Capability::StencilTest);

		return Ok(());

	}

	// TODO
	fn cam_look(&mut self, yaw: f32, pitch: f32) {
		self.cam_3d.set_angle(yaw, pitch);
		self.cur_shader_3d.send("view", self.cam_3d.as_mat());
	}

	fn cam_pos(&mut self, pos: Vec3) {
		self.cam_3d.set_pos(pos);
		self.cur_shader_3d.send("view", self.cam_3d.as_mat());
	}

	fn cam_front(&self) -> Vec3 {
		return self.cam_3d.front;
	}

}

#[derive(Clone, PartialEq)]
pub struct Texture {
	handle: Rc<gl::Texture>,
	width: u32,
	height: u32,
}

impl Texture {

	pub(super) fn from_handle(handle: gl::Texture, w: u32, h: u32) -> Self {
		return Self {
			handle: Rc::new(handle),
			width: w,
			height: h,
		};
	}

	#[cfg(feature = "img")]
	pub fn from_image(ctx: &Ctx, img: Image) -> Result<Self> {

		let w = img.width();
		let h = img.height();
		let handle = gl::Texture::init(&ctx.gl, w, h, &img.into_raw())?;

		return Ok(Self::from_handle(handle, w as u32, h as u32));

	}

	#[cfg(feature = "img")]
	pub fn from_file(ctx: &Ctx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_image(ctx, Image::from_file(path)?);
	}

	#[cfg(feature = "img")]
	pub fn from_bytes(ctx: &Ctx, data: &[u8]) -> Result<Self> {
		return Self::from_image(ctx, Image::from_bytes(data)?);
	}

	pub fn from_pixels(ctx: &Ctx, w: u32, h: u32, pixels: &[u8]) -> Result<Self> {

		let handle = gl::Texture::init(&ctx.gl, w, h, &pixels)?;
		return Ok(Self::from_handle(handle, w, h));

	}

	pub fn width(&self) -> u32 {
		return self.width;
	}

	pub fn height(&self) -> u32 {
		return self.height;
	}

	pub fn data(&mut self, x: u32, y: u32, width: u32, height: u32, data: &[u8]) {
		self.width = width;
		self.height = height;
		self.handle.data(x, y, width, height, data);
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
		let tw = tex.width() as i32;
		let th = tex.height() as i32;

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

	pub fn send(&self, name: &str, value: impl gl::UniformValue) {
		self.handle.send(name, value);
	}

}

#[derive(Clone, PartialEq)]
pub struct Canvas {

	handle: Rc<gl::Framebuffer>,
	pub(super) tex: Texture,

}

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

	pub fn width(&self) -> u32 {
		return self.tex.width();
	}

	pub fn height(&self) -> u32 {
		return self.tex.height();
	}

	#[cfg(feature = "img")]
	pub fn capture(&self, path: impl AsRef<Path>) -> Result<()> {

		let tex = &self.tex;
		let buffer = tex.handle.get_data(self.width(), self.height());

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
	normal: Vec3,
	color: Color,
}

impl Vertex3D {
	fn new(pos: Vec3, normal: Vec3, color: Color) -> Self {
		return Self {
			pos: pos,
			normal: normal,
			color: color,
		};
	}
}

impl VertexLayout for Vertex3D {

	const STRIDE: usize = 10;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.pos.z,
			self.normal.x,
			self.normal.y,
			self.normal.z,
			self.color.r,
			self.color.g,
			self.color.b,
			self.color.a,
		]);
	}

	fn attrs() -> gl::VertexAttrGroup {

		return gl::VertexAttrGroup::build()
			.add("pos", 3)
			.add("normal", 3)
			.add("color", 4)
			;

	}

}

#[derive(Clone)]
pub(super) struct Camera {
	front: Vec3,
	pos: Vec3,
}

impl Camera {

	pub fn new(pos: Vec3, yaw: f32, pitch: f32) -> Self {

		let mut c = Self {
			front: vec3!(),
			pos: vec3!(),
		};

		c.set_pos(pos);
		c.set_angle(yaw, pitch);

		return c;

	}

	pub(super) fn as_mat(&self) -> Mat4 {
		return math::lookat(self.pos, self.pos + self.front, vec3!(0, 1, 0));
	}

	pub fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
	}

	pub fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.front.x = pitch.cos() * (yaw + 90f32.to_radians()).cos();
		self.front.y = pitch.sin();
		self.front.z = pitch.cos() * (yaw + 90f32.to_radians()).sin();
		self.front = self.front.unit();

	}

}

#[derive(Clone)]
pub struct Model {
	pub(super) renderer: Rc<gl::Renderer<Vertex3D>>,
}

impl Model {

	fn from_tobj(ctx: &Ctx, tobj: tobj::LoadResult) -> Result<Self> {

		let (models, mtls) = tobj?;
		let mesh = &models.get(0).ok_or(Error::ObjLoad)?.mesh;
		let positions = &mesh.positions;
		let normals = &mesh.normals;
		let indices = &mesh.indices;
		let count = positions.len() / 3;

		// TODO: calculate normals
		let mut verts = Vec::with_capacity(count * Vertex3D::STRIDE);

		for i in 0..count {

			let vx = positions[i * 3 + 0];
			let vy = positions[i * 3 + 1];
			let vz = positions[i * 3 + 2];
			let nx = normals.get(i * 3 + 0).unwrap_or(&0.0);
			let ny = normals.get(i * 3 + 1).unwrap_or(&0.0);
			let nz = normals.get(i * 3 + 2).unwrap_or(&0.0);
			let vert = Vertex3D::new(vec3!(vx, vy, vz), vec3!(*nx, *ny, *nz), color!(rand!(), rand!(), rand!(), 1));

			vert.push(&mut verts);

		}

		let renderer = gl::Renderer::new(&ctx.gl, &verts, indices)?;

		return Ok(Self {
			renderer: Rc::new(renderer),
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

use glyph_brush::GlyphBrush;
use glyph_brush::BrushAction;
use glyph_brush::GlyphBrushBuilder;
use glyph_brush::Section;
use glyph_brush::rusttype;

#[derive(Clone)]
struct FontQuad {
	pos: Vec2,
	quad: Quad,
}

pub struct TrueTypeFont {
	cache: GlyphBrush<'static, FontQuad>,
	tex: Texture,
	quads: Vec<FontQuad>,
	size: f32,
}

impl TrueTypeFont {

	pub fn new(ctx: &Ctx, bytes: &'static [u8], size: f32) -> Result<Self> {

		let font_cache = GlyphBrushBuilder::using_font_bytes(bytes).build();

		let (width, height) = font_cache.texture_dimensions();
		let font_cache_texture = gl::Texture::new(&ctx.gl, width, height)?;

		return Ok(Self {
			cache: font_cache,
			tex: Texture::from_handle(font_cache_texture, width, height),
			quads: Vec::with_capacity(64),
			size: size,
		})

	}

	// TODO: let shape take care of this
	pub fn draw(&mut self, ctx: &mut Ctx, txt: &str) -> Result<()> {

		let mut tex = self.tex.clone();

		self.cache.queue(Section {
			text: txt,
			scale: rusttype::Scale::uniform(self.size),
			..Section::default()
		});

		let mut update_texture = |rect: rusttype::Rect<u32>, data: &[u8]| {

			let mut padded_data = Vec::with_capacity(data.len() * 4);

			for a in data {
				padded_data.extend_from_slice(&[
					255,
					255,
					255,
					*a,
				]);
			}

			tex.data(
				rect.min.x,
				rect.min.y,
				rect.width(),
				rect.height(),
				&padded_data,
			);

		};

		let into_vertex = |verts: &glyph_brush::GlyphVertex| {

			let uv = verts.tex_coords;
			let pos = verts.pixel_coords.min;
			let x = uv.min.x;
			let y = uv.min.y;
			let w = uv.max.x - x;
			let h = uv.max.y - y;

			return FontQuad {
				pos: vec2!(pos.x, pos.y),
				quad: quad!(x, y, w, h),
			}

		};

		if let Ok(action) = self.cache.process_queued(
			|rect, tex_data| update_texture(rect, tex_data),
			|verts| into_vertex(&verts),
		) {

			if let BrushAction::Draw(quads) = action {
				self.quads = quads;
			}

			for q in &self.quads {

				ctx.push();
				ctx.translate(q.pos);
				ctx.draw(shapes::sprite(&tex).quad(q.quad))?;
				ctx.pop()?;

			}

		}

		return Ok(());

	}

}

pub(super) enum ActiveShader {
	Default,
	User(Shader),
}

pub trait DrawCmd {
	fn draw(&self, ctx: &mut Ctx) -> Result<()>;
}

