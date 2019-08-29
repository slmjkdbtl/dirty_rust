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
pub use gl::FilterMode;
pub use gl::Surface;
pub use gl::Cmp;

pub trait Gfx {

	// clearing
	fn clear(&mut self);
	fn clear_ex(&mut self, s: Surface);
	fn clear_color(&self, c: Color);

	// stats
	fn draw_calls(&self) -> usize;

	// drawing
	fn draw(&mut self, t: impl Drawable) -> Result<()>;
	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_with(&mut self, shader: &Shader, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_masked_ex(&mut self, f1: Cmp, f2: Cmp, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// transform
	fn push(&mut self, t: &Transform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// coord
	fn coord(&self, coord: Origin) -> Vec2;

	// camera
	fn use_cam(&mut self, cam: &Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

}

impl Gfx for Ctx {

	fn clear(&mut self) {

		flush(self);
		self.gl.clear(Surface::Color);
		self.gl.clear(Surface::Depth);
		self.gl.clear(Surface::Stencil);

	}

	fn clear_ex(&mut self, s: Surface) {

		flush(self);
		self.gl.clear(s);

	}

	fn clear_color(&self, c: Color) {
		self.gl.clear_color(c);
	}

	fn draw_calls(&self) -> usize {
		return self.draw_calls_last;
	}

	fn push(&mut self, t: &Transform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		self.transform_stack.push(self.transform.clone());
		self.transform = self.transform.apply(*t);

		f(self)?;

		self.transform = self.transform_stack
			.pop()
			.ok_or(Error::Gfx("failed to pop transform stack".into()))?;

		return Ok(());

	}

	fn draw(&mut self, thing: impl Drawable) -> Result<()> {
		return thing.draw(self);
	}

	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		flush(self);
		self.proj_2d = flip_matrix(&self.proj_2d);
		self.proj_3d = flip_matrix(&self.proj_3d);
		self.cur_shader_2d.send("proj", self.proj_2d);
		self.cur_shader_3d.send("proj", self.proj_3d);
		self.gl.viewport(0, 0, canvas.width(), canvas.height());

		// TODO: fixed fullscreen framebuffer weirdness, but now weird resize
		// TODO: what if shader is changed in callback?
		canvas.handle.with(|| -> Result<()> {

			let t = self.transform;

			self.transform = Transform::new();
			f(self)?;
			self.transform = t;
			flush(self);

			return Ok(());

		})?;

		self.gl.viewport(0, 0, self.width() * self.dpi() as i32, self.height() * self.dpi() as i32);
		self.proj_2d = flip_matrix(&self.proj_2d);
		self.proj_3d = flip_matrix(&self.proj_3d);
		self.cur_shader_2d.send("proj", self.proj_2d);
		self.cur_shader_3d.send("proj", self.proj_3d);

		return Ok(());

	}

	fn draw_with(&mut self, shader: &Shader, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		self.cur_shader_2d = shader.clone();
		self.cur_shader_2d.send("proj", self.proj_2d);
		f(self)?;
		// TODO: why is this flush necessary?
		flush(self);
		self.cur_shader_2d = self.default_shader_2d.clone();

		return Ok(());

	}

	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
		return self.draw_masked_ex(Cmp::Never, Cmp::Equal, mask, draw);
	}

	// TODO: use gl::StencilDraw
	fn draw_masked_ex(&mut self, f1: Cmp, f2: Cmp, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

// 		let d1 = gl::StencilDraw {
// 			ops: gl::StencilOps {
// 				sfail: gl::StencilOp::Replace,
// 				dpfail: gl::StencilOp::Replace,
// 				dppass: gl::StencilOp::Replace,
// 			},
// 			func: gl::StencilFunc {
// 				cmp: gl::Cmp::Never,
// 				rf: 1,
// 				mask: 0xff,
// 			},
// 		};

		flush(self);
		self.gl.clear(gl::Surface::Stencil);
		self.gl.enable(gl::Capability::StencilTest);
		self.gl.stencil_func(f1);
		self.gl.stencil_op(gl::StencilOp::Replace, gl::StencilOp::Replace, gl::StencilOp::Replace);

		mask(self)?;
		flush(self);
		self.gl.stencil_func(f2);
		self.gl.stencil_op(gl::StencilOp::Keep, gl::StencilOp::Keep, gl::StencilOp::Keep);
		draw(self)?;
		flush(self);
		self.gl.disable(gl::Capability::StencilTest);

		return Ok(());

	}

	fn coord(&self, coord: Origin) -> Vec2 {

		let w = self.width();
		let h = self.height();
		let orig_pt = self.conf.origin.as_pt();
		let coord_pt = coord.as_pt();

		return (coord_pt - orig_pt) / 2.0 * vec2!(w, h);

	}

	fn use_cam(&mut self, cam: &Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
		self.cur_shader_3d.send("view", cam.get_lookat_matrix());
		return f(self);
	}

}

fn flip_matrix(m: &Mat4) -> Mat4 {

	let mut nm = m.clone();

	if let Some(val) = nm.get_mut(1, 1) {
		*val = -*val;
	}

	if let Some(val) = nm.get_mut(3, 1) {
		*val = -*val;
	}

	return nm;

}

pub(super) fn begin(ctx: &mut Ctx) {

	ctx.draw_calls_last = ctx.draw_calls;
	ctx.draw_calls = 0;
	ctx.clear();

}

pub(super) fn end(ctx: &mut Ctx) {

	flush(ctx);
	ctx.transform = Transform::new();
	ctx.transform_stack.clear();
	ctx.draw_calls += ctx.renderer_2d.draw_count();
	ctx.renderer_2d.clear();

}

pub(super) fn flush(ctx: &mut Ctx) {
	ctx.renderer_2d.flush();
}

pub struct Vertex2D {
	pos: Vec2,
	uv: Vec2,
	color: Color,
}

impl Vertex2D {
	pub fn new(pos: Vec2, uv: Vec2, color: Color) -> Self {
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

pub struct Vertex3D {
	pos: Vec3,
	normal: Vec3,
	color: Color,
}

impl Vertex3D {
	pub fn new(pos: Vec3, normal: Vec3, color: Color) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flip {
	None,
	X,
	Y,
	XY,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Origin {
	TopLeft,
	Top,
	TopRight,
	Left,
	Center,
	Right,
	BottomLeft,
	Bottom,
	BottomRight,
}

impl Origin {

	pub fn to_ortho(&self, w: i32, h: i32) -> Mat4 {

		use Origin::*;

		let w = w as f32;
		let h = h as f32;
		let near = -1.0;
		let far = 1.0;

		return match self {
			TopLeft => ortho(0.0, w, h, 0.0, near, far),
			Top => ortho(-w / 2.0, w / 2.0, h, 0.0, near, far),
			TopRight => ortho(-w, 0.0, h, 0.0, near, far),
			Left => ortho(0.0, w, h / 2.0, -h / 2.0, near, far),
			Center => ortho(-w / 2.0, w / 2.0, h / 2.0, -h / 2.0, near, far),
			Right => ortho(-w, 0.0, h / 2.0, -h / 2.0, near, far),
			BottomLeft => ortho(0.0, w, 0.0, -h, near, far),
			Bottom => ortho(-w / 2.0, w / 2.0, 0.0, -h, near, far),
			BottomRight => ortho(-w, 0.0, 0.0, -h, near, far),
		};

	}

	pub fn as_pt(&self) -> Vec2 {

		use Origin::*;

		return match self {
			TopLeft => vec2!(-1, -1),
			Top => vec2!(0, -1),
			TopRight => vec2!(1, -1),
			Left => vec2!(-1, 0),
			Center => vec2!(0, 0),
			Right => vec2!(1, 0),
			BottomLeft => vec2!(-1, 1),
			Bottom => vec2!(0, 1),
			BottomRight => vec2!(1, 1),
		};

	}

}

#[derive(Clone, PartialEq)]
pub struct Tex2D {
	pub(super) handle: Rc<gl::Texture>,
	width: i32,
	height: i32,
}

impl Tex2D {

	pub(super) fn from_handle(handle: gl::Texture, w: i32, h: i32) -> Self {
		return Self {
			handle: Rc::new(handle),
			width: w,
			height: h,
		};
	}

	#[cfg(feature = "img")]
	pub fn from_img(ctx: &Ctx, img: Image) -> Result<Self> {

		let w = img.width();
		let h = img.height();
		let handle = gl::Texture::init(&ctx.gl, w, h, &img.into_raw())?;

		handle.filter(ctx.conf.texture_filter);

		return Ok(Self::from_handle(handle, w as i32, h as i32));

	}

	#[cfg(feature = "img")]
	pub fn from_file(ctx: &Ctx, path: impl AsRef<Path>) -> Result<Self> {
		return Self::from_img(ctx, Image::from_file(path)?);
	}

	#[cfg(feature = "img")]
	pub fn from_bytes(ctx: &Ctx, data: &[u8]) -> Result<Self> {
		return Self::from_img(ctx, Image::from_bytes(data)?);
	}

	// TODO: use Color
	pub fn from_pixels(ctx: &Ctx, w: i32, h: i32, pixels: &[u8]) -> Result<Self> {

		let handle = gl::Texture::init(&ctx.gl, w, h, &pixels)?;
		handle.filter(ctx.conf.texture_filter);
		return Ok(Self::from_handle(handle, w, h));

	}

	pub fn width(&self) -> i32 {
		return self.width;
	}

	pub fn height(&self) -> i32 {
		return self.height;
	}

	pub fn data(&mut self, x: i32, y: i32, width: i32, height: i32, data: &[u8]) {
		self.width = width;
		self.height = height;
		self.handle.data(x, y, width, height, data);
	}

}

/// bitmap font
#[derive(Clone, PartialEq)]
pub struct Font {

	pub(super) tex: Tex2D,
	pub(super) map: HashMap<char, Quad>,
	pub(super) quad_size: Vec2,
	grid_width: i32,
	grid_height: i32,

}

impl Font {

	/// creat a bitmap font from a texture, and grid of characters
	pub fn from_tex(tex: Tex2D, cols: usize, rows: usize, chars: &str) -> Result<Self> {

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
			grid_width: tw as i32 / cols as i32,
			grid_height: th as i32 / rows as i32,

		});

	}

	/// get current font width for string
	pub fn width(&self) -> i32 {
		return self.grid_width;
	}

	/// get current text height
	pub fn height(&self) -> i32 {
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
	pub(super) tex: Tex2D,

}

impl Canvas {

	pub fn new(ctx: &Ctx, width: i32, height: i32) -> Result<Self> {

		let dpi = ctx.dpi();
		let tw = (width as f64 * dpi) as i32;
		let th = (height as f64 * dpi) as i32;
		let pixels = vec![0.0 as u8; (tw * th * 4) as usize];
		let tex = Tex2D::from_pixels(&ctx, tw, th, &pixels)?;
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

#[derive(Clone)]
pub struct Camera {
	front: Vec3,
	pos: Vec3,
	yaw: f32,
	pitch: f32,
}

impl Camera {

	pub fn new(pos: Vec3, yaw: f32, pitch: f32) -> Self {

		let mut c = Self {
			pos: vec3!(),
			front: vec3!(),
			yaw: 0.0,
			pitch: 0.0,
		};

		c.set_pos(pos);
		c.set_angle(yaw, pitch);

		return c;

	}

	pub fn set_pos(&mut self, pos: Vec3) {
		self.pos = pos;
	}

	pub fn set_angle(&mut self, yaw: f32, pitch: f32) {

		self.yaw = yaw;
		self.pitch = pitch;

		self.front = vec3!(
			self.pitch.cos() * (self.yaw + 90f32.to_radians()).cos(),
			self.pitch.sin(),
			self.pitch.cos() * (self.yaw + 90f32.to_radians()).sin(),
		).normalize();

	}

	pub fn front(&self) -> Vec3 {
		return self.front;
	}

	pub fn pos(&self) -> Vec3 {
		return self.pos;
	}

	pub fn yaw(&self) -> f32 {
		return self.yaw;
	}

	pub fn pitch(&self) -> f32 {
		return self.pitch;
	}

	pub(super) fn get_lookat_matrix(&self) -> Mat4 {
		return math::lookat(self.pos, self.pos + self.front, vec3!(0, 1, 0));
	}

}

fn get_vertex_normals(pos: &[f32], indices: &[u32]) -> Vec<Vec3> {

	let vert_count = pos.len() / 3;
	let tri_count = indices.len() / 3;
	let mut normals = vec![vec3!(0); vert_count];

	for i in 0..tri_count {

		let i1 = indices[i * 3] as usize;
		let i2 = indices[i * 3 + 1] as usize;
		let i3 = indices[i * 3 + 2] as usize;
		let v1 = vec3!(pos[i1 * 3], pos[i1 * 3 + 1], pos[i1 * 3 + 2]);
		let v2 = vec3!(pos[i2 * 3], pos[i2 * 3 + 1], pos[i2 * 3 + 2]);
		let v3 = vec3!(pos[i3 * 3], pos[i3 * 3 + 1], pos[i3 * 3 + 2]);
		let normal = Vec3::cross((v3 - v1), (v2 - v1)).normalize();

		normals[i1] += normal;
		normals[i2] += normal;
		normals[i3] += normal;

	}

	for n in &mut normals {
		*n = n.normalize();
	}

	return normals;

}

#[derive(Clone)]
pub struct Model {
	pub(super) renderer: Rc<gl::Renderer<Vertex3D>>,
}

impl Model {

	fn from_tobj(ctx: &Ctx, tobj: tobj::LoadResult) -> Result<Self> {

		let (models, _) = tobj?;
		let mesh = &models.get(0).ok_or(Error::ObjLoad)?.mesh;
		let positions = &mesh.positions;
		let indices = &mesh.indices;
		let vert_count = positions.len() / 3;
		let normals = get_vertex_normals(&positions, &indices);
		let mut verts = Vec::with_capacity(vert_count * Vertex3D::STRIDE);

		for i in 0..vert_count {

			let vx = positions[i * 3 + 0];
			let vy = positions[i * 3 + 1];
			let vz = positions[i * 3 + 2];
			let vert = Vertex3D::new(vec3!(vx, vy, vz), normals[i], color!(rand!(), rand!(), rand!(), 1));

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

pub(super) struct CubeShape;

impl Shape for CubeShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 8;

	fn push(&self, queue: &mut Vec<f32>) {

		Self::Vertex::new(vec3!(-0.5, -0.5, 0.5), vec3!(0.33, 0.33, -0.66), color!(1, 0, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, -0.5, 0.5), vec3!(-0.66, 0.66, -0.33), color!(0, 1, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, 0.5, 0.5), vec3!(-0.33, -0.33, -0.66), color!(0, 0, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, 0.5, 0.5), vec3!(0.66, -0.66, -0.33), color!(1, 1, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, -0.5, -0.5), vec3!(0.66, 0.66, 0.33), color!(1, 0, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, -0.5, -0.5), vec3!(-0.33, 0.33, 0.66), color!(0, 1, 0, 1)).push(queue);
		Self::Vertex::new(vec3!(0.5, 0.5, -0.5), vec3!(-0.66, -0.66, 0.33), color!(0, 0, 1, 1)).push(queue);
		Self::Vertex::new(vec3!(-0.5, 0.5, -0.5), vec3!(0.33, -0.33, 0.66), color!(1, 1, 1, 1)).push(queue);

// 		Self::Vertex::new(vec3!(-0.5, -0.5, 0.5), vec3!(0.33, 0.33, -0.66), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(0.5, -0.5, 0.5), vec3!(-0.66, 0.66, -0.33), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(0.5, 0.5, 0.5), vec3!(-0.33, -0.33, -0.66), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(-0.5, 0.5, 0.5), vec3!(0.66, -0.66, -0.33), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(-0.5, -0.5, -0.5), vec3!(0.66, 0.66, 0.33), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(0.5, -0.5, -0.5), vec3!(-0.33, 0.33, 0.66), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(0.5, 0.5, -0.5), vec3!(-0.66, -0.66, 0.33), color!(1)).push(queue);
// 		Self::Vertex::new(vec3!(-0.5, 0.5, -0.5), vec3!(0.33, -0.33, 0.66), color!(1)).push(queue);

	}

	fn indices() -> Vec<u32> {
		return vec![
			0, 1, 2,
			2, 3, 0,
			1, 5, 6,
			6, 2, 1,
			7, 6, 5,
			5, 4, 7,
			4, 0, 3,
			3, 7, 4,
			4, 5, 1,
			1, 0, 4,
			3, 2, 6,
			6, 7, 3,
		];
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
	tex: Tex2D,
	quads: Vec<FontQuad>,
	size: f32,
}

impl TrueTypeFont {

	pub fn new(ctx: &Ctx, bytes: &'static [u8], size: f32) -> Result<Self> {

		let font_cache = GlyphBrushBuilder::using_font_bytes(bytes).build();

		let (width, height) = font_cache.texture_dimensions();
		let font_cache_texture = gl::Texture::new(&ctx.gl, width as i32, height as i32)?;

		return Ok(Self {
			cache: font_cache,
			tex: Tex2D::from_handle(font_cache_texture, width as i32, height as i32),
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
				rect.min.x as i32,
				rect.min.y as i32,
				rect.width() as i32,
				rect.height() as i32,
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
				ctx.push(&t()
					.translate(q.pos), |ctx| {
					return ctx.draw(shapes::sprite(&tex).quad(q.quad));
				})?;
			}

		}

		return Ok(());

	}

}

#[derive(Clone, Copy, Default)]
pub struct Transform {
	matrix: Mat4,
}

impl Transform {

	pub fn new() -> Self {
		return Self::from_mat4(Mat4::identity());
	}

	pub fn from_mat4(m: Mat4) -> Self {
		return Self {
			matrix: m,
		};
	}

	pub fn translate(mut self, p: Vec2) -> Self {
		self.matrix *= Mat4::translate(vec3!(p.x, p.y, 0.0));
		return self;
	}

	pub fn rotate(mut self, a: f32) -> Self {
		self.matrix *= Mat4::rotate(a, vec3!(0, 0, 1));
		return self;
	}

	pub fn scale(mut self, s: Vec2) -> Self {
		self.matrix *= Mat4::scale(vec3!(s.x, s.y, 1.0));
		return self;
	}

	pub fn translate_3d(mut self, p: Vec3) -> Self {
		self.matrix *= Mat4::translate(p);
		return self;
	}

	pub fn scale_3d(mut self, s: Vec3) -> Self {
		self.matrix *= Mat4::scale(s);
		return self;
	}

	pub fn rotate_x(mut self, a: f32) -> Self {
		self.matrix *= Mat4::rotate(a, vec3!(1, 0, 0));
		return self;
	}

	pub fn rotate_y(mut self, a: f32) -> Self {
		self.matrix *= Mat4::rotate(a, vec3!(0, 1, 0));
		return self;
	}

	pub fn rotate_z(mut self, a: f32) -> Self {
		self.matrix *= Mat4::rotate(a, vec3!(0, 0, 1));
		return self;
	}

	pub fn matrix(&self) -> Mat4 {
		return self.matrix;
	}

	pub fn apply(self, other: Self) -> Self {
		return Self::from_mat4(self.matrix * other.matrix);
	}

}

impl gfx::UniformValue for Transform {
	fn get(&self) -> gfx::UniformType {
		return gfx::UniformValue::get(&self.matrix);
	}
}

pub fn t() -> Transform {
	return Transform::new();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineJoin {
	None,
	Round,
	Bevel,
	Miter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineCap {
	Square,
	Butt,
	Round,
}

pub trait Drawable {
	fn draw(&self, ctx: &mut Ctx) -> Result<()>;
}

