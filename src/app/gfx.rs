// wengwengweng

use std::mem;

use crate::*;
use crate::math::*;
use super::*;

pub use gl::VertexLayout;
pub use gl::Shape;

pub use gl::UniformType;
pub use gl::UniformValues;
pub use gl::FilterMode;
pub use gl::Surface;
pub use gl::Cmp;

pub use res::CP437_IMG;
pub use res::CP437_COLS;
pub use res::CP437_ROWS;
pub use res::CP437_CHARS;

pub use res::PROGGY_IMG;
pub use res::PROGGY_COLS;
pub use res::PROGGY_ROWS;
pub use res::PROGGY_CHARS;

pub use texture::*;
pub use transform::*;
pub use shader::*;
pub use canvas::*;
pub use font::*;
pub use camera::*;
pub use mesh::*;

pub trait Gfx {

	// clearing
	fn clear(&mut self);
	fn clear_ex(&mut self, s: Surface);

	// stats
	fn draw_calls(&self) -> usize;

	// drawing
	fn draw(&mut self, t: impl Drawable) -> Result<()>;
	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_2d_with<U: Uniform>(&mut self, shader: &Shader2D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_3d_with<U: Uniform>(&mut self, shader: &Shader3D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_masked_ex(&mut self, f1: Cmp, f2: Cmp, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// transform
	fn push(&mut self, t: &Transform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// coord
	fn coord(&self, coord: Origin) -> Vec2;

	// camera
	fn use_cam(&mut self, cam: &impl Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// query
	fn default_font(&self) -> &BitmapFont;
	fn transform(&self) -> Transform;

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

	fn draw_calls(&self) -> usize {
		return self.draw_calls_last;
	}

	fn push(&mut self, t: &Transform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let ot = self.transform;

		self.transform = ot.apply(t);
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	fn draw(&mut self, thing: impl Drawable) -> Result<()> {
		return thing.draw(self);
	}

	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		if self.cur_canvas.is_some() {
			return Err(Error::Gfx(format!("cannot use canvas inside a canvas call")));
		}

		flush(self);

		let o_proj_2d = self.proj_2d;
		let o_proj_3d = self.proj_3d;
		let t = self.transform;

		self.gl.viewport(0, 0, canvas.width(), canvas.height());
		self.cur_canvas = Some(canvas.clone());

		self.proj_2d = flip_matrix(&o_proj_2d);
		self.proj_3d = flip_matrix(&o_proj_3d);
		self.transform = Transform::new();

		canvas.handle.with(|| -> Result<()> {
			f(self)?;
			return Ok(());
		})?;

		flush(self);

		self.transform = t;
		self.proj_2d = o_proj_2d;
		self.proj_3d = o_proj_3d;

		self.cur_canvas = None;
		self.gl.viewport(0, 0, self.width() * self.dpi() as i32, self.height() * self.dpi() as i32);

		return Ok(());

	}

	fn draw_2d_with<U: Uniform>(&mut self, shader: &Shader2D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		flush(self);
		self.cur_pipeline_2d = gl::Pipeline::clone(&shader.handle);
		self.cur_custom_uniform_2d = Some(uniform.values());
		f(self)?;
		flush(self);
		self.cur_pipeline_2d = self.default_pipeline_2d.clone();
		self.cur_custom_uniform_2d = None;

		return Ok(());

	}

	fn draw_3d_with<U: Uniform>(&mut self, shader: &Shader3D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		flush(self);
		self.cur_pipeline_3d = gl::Pipeline::clone(&shader.handle);
		self.cur_custom_uniform_3d = Some(uniform.values());
		f(self)?;
		flush(self);
		self.cur_pipeline_3d = self.default_pipeline_3d.clone();
		self.cur_custom_uniform_3d = None;

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

	fn use_cam(&mut self, cam: &impl Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let oview_3d = self.view_3d;
		let oproj_3d = self.proj_3d;

		self.view_3d = cam.lookat();
		self.proj_3d = cam.projection();

		if self.cur_canvas.is_some() {
			self.proj_3d = flip_matrix(&self.proj_3d);
		}

		f(self)?;

		self.view_3d = oview_3d;
		self.proj_3d = oproj_3d;

		return Ok(());

	}

	fn default_font(&self) -> &BitmapFont {
		return &self.default_font;
	}

	fn transform(&self) -> Transform {
		return self.transform;
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
	ctx.draw_calls += ctx.renderer_2d.draw_count();
	ctx.draw_calls += ctx.renderer_3d.draw_count();
	ctx.renderer_2d.clear();
	ctx.renderer_3d.clear();

}

pub(super) fn flush(ctx: &mut Ctx) {
	ctx.renderer_2d.flush();
	ctx.renderer_3d.flush();
}

#[derive(Clone)]
pub struct Vertex2D {
	pub pos: Vec2,
	pub uv: Vec2,
	pub color: Color,
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
		return &[
			("a_pos", 2),
			("a_uv", 2),
			("a_color", 4),
		];
	}

}

#[derive(Clone)]
pub struct Vertex3D {
	pub pos: Vec3,
	pub uv: Vec2,
	pub normal: Vec3,
	pub color: Color,
}

impl VertexLayout for Vertex3D {

	const STRIDE: usize = 12;

	fn push(&self, queue: &mut Vec<f32>) {
		queue.extend_from_slice(&[
			self.pos.x,
			self.pos.y,
			self.pos.z,
			self.uv.x,
			self.uv.y,
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
		return &[
			("a_pos", 3),
			("a_uv", 2),
			("a_normal", 3),
			("a_color", 4),
		];
	}

}

pub(super) struct QuadShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub flip: Flip,
}

impl Shape for QuadShape {

	type Vertex = Vertex2D;
	const COUNT: usize = 4;

	fn vertices(&self, queue: &mut Vec<f32>) {

		let t = self.transform;
		let q = self.quad;
		let c = self.color;
		let p1 = t * (vec2!(-0.5, 0.5));
		let p2 = t * (vec2!(0.5, 0.5));
		let p3 = t * (vec2!(0.5, -0.5));
		let p4 = t * (vec2!(-0.5, -0.5));

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

		Vertex2D {
			pos: p1,
			uv: u1,
			color: c
		}.push(queue);

		Vertex2D {
			pos: p2,
			uv: u2,
			color: c
		}.push(queue);

		Vertex2D {
			pos: p3,
			uv: u3,
			color: c
		}.push(queue);

		Vertex2D {
			pos: p4,
			uv: u4,
			color: c
		}.push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[0, 1, 3, 1, 2, 3];
	}

}

#[derive(Clone, Copy, Debug)]
pub enum ScaleMode {
	Letterbox,
	Stretch,
}

#[derive(Debug, Clone, Copy)]
pub enum Flip {
	None,
	X,
	Y,
	XY,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NormalMode {
	Vertex,
	Surface,
}

// TODO: messy
pub(super) struct FlagShape {
	pub transform: Mat4,
	pub quad: Quad,
	pub color: Color,
	pub flip: Flip,
}

impl Shape for FlagShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 4;

	fn vertices(&self, queue: &mut Vec<f32>) {

		let t = self.transform;
		let q = self.quad;
		let c = self.color;
		let p1 = t * (vec2!(-0.5, 0.5));
		let p2 = t * (vec2!(0.5, 0.5));
		let p3 = t * (vec2!(0.5, -0.5));
		let p4 = t * (vec2!(-0.5, -0.5));

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

		Vertex3D {
			pos: vec3!(p1.x, p1.y, 0.0),
			uv: u1,
			normal: vec3!(0, 0, -1),
			color: c,
		}.push(queue);

		Vertex3D {
			pos: vec3!(p2.x, p2.y, 0.0),
			uv: u2,
			normal: vec3!(0, 0, -1),
			color: c,
		}.push(queue);

		Vertex3D {
			pos: vec3!(p3.x, p3.y, 0.0),
			uv: u3,
			normal: vec3!(0, 0, -1),
			color: c,
		}.push(queue);

		Vertex3D {
			pos: vec3!(p4.x, p4.y, 0.0),
			uv: u4,
			normal: vec3!(0, 0, -1),
			color: c,
		}.push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[0, 1, 3, 1, 2, 3];
	}

}

pub(super) struct CubeShape;

impl Shape for CubeShape {

	type Vertex = Vertex3D;
	const COUNT: usize = 8;

	fn vertices(&self, queue: &mut Vec<f32>) {

		Vertex3D {
			pos: vec3!(-0.5, -0.5, 0.5),
			uv: vec2!(),
			normal: vec3!(-0.41, -0.41, 0.82),
			color: color!(1, 0, 0, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(0.5, -0.5, 0.5),
			uv: vec2!(),
			normal: vec3!(0.67, -0.67, 0.33),
			color: color!(0, 1, 0, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(0.5, 0.5, 0.5),
			uv: vec2!(),
			normal: vec3!(0.41, 0.41, 0.82),
			color: color!(0, 0, 1, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(-0.5, 0.5, 0.5),
			uv: vec2!(),
			normal: vec3!(-0.67, 0.67, 0.33),
			color: color!(1, 1, 1, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(-0.5, -0.5, -0.5),
			uv: vec2!(),
			normal: vec3!(-0.67, -0.67, -0.33),
			color: color!(1, 0, 0, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(0.5, -0.5, -0.5),
			uv: vec2!(),
			normal: vec3!(0.41, -0.41, -0.82),
			color: color!(0, 1, 0, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(0.5, 0.5, -0.5),
			uv: vec2!(),
			normal: vec3!(0.67, 0.67, -0.33),
			color: color!(0, 0, 1, 1),
		}.push(queue);

		Vertex3D {
			pos: vec3!(-0.5, 0.5, -0.5),
			uv: vec2!(),
			normal: vec3!(-0.41, 0.41, -0.82),
			color: color!(1, 1, 1, 1),
		}.push(queue);

	}

	fn indices() -> &'static [u32] {
		return &[
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

