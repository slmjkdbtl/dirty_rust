// wengwengweng

//! Graphics

use crate::*;
use crate::math::*;
use super::*;

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
pub use model::*;
pub use desc::*;

pub trait Action = FnOnce(&mut Ctx) -> Result<()>;

pub trait Gfx {

	// clearing
	fn clear(&mut self);
	fn clear_ex(&mut self, s: Surface);

	// stats
	fn draw_calls(&self) -> usize;

	// drawing
	fn draw(&mut self, t: &impl Drawable) -> Result<()>;
	fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_2d_with<U: Uniform>(&mut self, shader: &Shader2D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_3d_with<U: Uniform>(&mut self, shader: &Shader3D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;
	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// transform
	fn push(&mut self, t: &Transform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// coord
	fn coord(&self, coord: Origin) -> Vec2;

	// camera
	fn use_cam(&mut self, cam: &impl Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()>;

	// query
	fn default_font(&self) -> &BitmapFont;
	fn transform(&self) -> Transform;
	fn to_sc(&self, pt: Vec3) -> Vec2;

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

	fn draw(&mut self, thing: &impl Drawable) -> Result<()> {
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

		canvas.gl_fbuf().with(|| -> Result<()> {
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
		self.cur_pipeline_2d = gl::Pipeline::clone(&shader.gl_pipeline());
		self.cur_custom_uniform_2d = Some(uniform.values());
		f(self)?;
		flush(self);
		self.cur_pipeline_2d = self.default_pipeline_2d.clone();
		self.cur_custom_uniform_2d = None;

		return Ok(());

	}

	fn draw_3d_with<U: Uniform>(&mut self, shader: &Shader3D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		flush(self);
		self.cur_pipeline_3d = gl::Pipeline::clone(&shader.gl_pipeline());
		self.cur_custom_uniform_3d = Some(uniform.values());
		f(self)?;
		flush(self);
		self.cur_pipeline_3d = self.default_pipeline_3d.clone();
		self.cur_custom_uniform_3d = None;

		return Ok(());

	}

	fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let gl = self.gl.clone();

		flush(self);
		gl.enable(gl::Capability::StencilTest);
		gl.clear(Surface::Stencil);

		gl.stencil(gl::StencilFunc {
			cmp: Cmp::Never,
			rf: 1,
			mask: 0xff,
		}, gl::StencilOps {
			sfail: gl::StencilOp::Replace,
			dpfail: gl::StencilOp::Replace,
			dppass: gl::StencilOp::Replace,
		}, || {
			return mask(self);
		})?;

		flush(self);

		gl.stencil(gl::StencilFunc {
			cmp: Cmp::Equal,
			rf: 1,
			mask: 0xff,
		}, gl::StencilOps {
			sfail: gl::StencilOp::Keep,
			dpfail: gl::StencilOp::Keep,
			dppass: gl::StencilOp::Keep,
		}, || {
			return draw(self);
		})?;

		flush(self);
		gl.disable(gl::Capability::StencilTest);

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

	fn to_sc(&self, pt: Vec3) -> Vec2 {

		let (gw, gh) = (self.gwidth(), self.gheight());
		let pt = self.proj_3d * self.view_3d * self.transform * pt;
		let pt = (pt / pt.z * 0.5).xy() * vec2!(gw, -gh);
		let pt = pt - self.conf.origin.as_pt() * vec2!(gw, gh) / 2.0;

		return pt;

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

pub trait Drawable {
	fn draw(&self, ctx: &mut Ctx) -> Result<()>;
}

