// wengwengweng

//! Graphics

use crate::*;
use crate::math::*;
use super::*;

pub use gl::UniformValues;
pub use gl::IntoUniformValue;
pub use gl::FilterMode;
pub use gl::Surface;
pub use gl::Cmp;

pub use texture::*;
pub use transform::*;
pub use shader::*;
pub use canvas::*;
pub use font::*;
pub use camera::*;
pub use model::*;
pub use desc::*;
pub use skybox::*;

pub trait Action = FnOnce(&mut Ctx) -> Result<()>;

impl Ctx {

	pub fn clear(&mut self) {

		flush(self);
		self.gl.clear(Surface::Color);
		self.gl.clear(Surface::Depth);
		self.gl.clear(Surface::Stencil);

	}

	pub fn clear_ex(&mut self, s: Surface) {

		flush(self);
		self.gl.clear(s);

	}

	pub fn draw_calls(&self) -> usize {
		return self.draw_calls_last;
	}

	pub fn push(&mut self, t: &Transform, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let ot = self.transform;

		self.transform = ot.apply(t);
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	pub fn reset(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let ot = self.transform;

		self.transform = Transform::new();
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	pub fn draw(&mut self, shape: &impl Drawable) -> Result<()> {
		return shape.draw(self);
	}

	pub fn draw_t(&mut self, t: &Transform, shape: &impl Drawable) -> Result<()> {
		return self.push(t, |ctx| {
			return ctx.draw(shape);
		});
	}

	pub fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		if self.cur_canvas.is_some() {
			return Err(Error::Gfx(format!("cannot use canvas inside a canvas")));
		}

		flush(self);

		let o_proj_2d = self.proj_2d;
		let o_proj_3d = self.proj_3d;
		let t = self.transform;
		let dpi = self.dpi();

		self.cur_canvas = Some(canvas.clone());

		let proj_2d = gfx::OrthoProj {
			width: canvas.width() as f32 / dpi,
			height: canvas.height() as f32 / dpi,
			near: self.conf.near,
			far: self.conf.far,
			origin: canvas.origin(),
		}.as_mat4();

		self.proj_2d = proj_2d.flip_y();
		self.proj_3d = o_proj_3d.flip_y();
		self.transform = Transform::new();

		self.gl.viewport(0, 0, canvas.width(), canvas.height());

		canvas.gl_fbuf().with(|| -> Result<()> {
			f(self)?;
			flush(self);
			return Ok(());
		})?;

		self.transform = t;
		self.proj_2d = o_proj_2d;
		self.proj_3d = o_proj_3d;

		self.cur_canvas = None;

		self.gl.viewport(
			0,
			0,
			(self.width as f32 * dpi) as i32,
			(self.height as f32 * dpi) as i32,
		);

		return Ok(());

	}

	pub fn draw_2d_with<U: Uniform>(&mut self, shader: &Shader2D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let uniforms = uniform.values()
			.into_iter()
			.map(|(n, v)| (n, v.into_uniform()))
			.collect::<Vec<(&'static str, gl::UniformValue)>>();

		flush(self);
		self.cur_pipeline_2d = gl::Pipeline::clone(&shader.gl_pipeline());
		self.cur_custom_uniform_2d = Some(uniforms);
		f(self)?;
		flush(self);
		self.cur_pipeline_2d = self.default_pipeline_2d.clone();
		self.cur_custom_uniform_2d = None;

		return Ok(());

	}

	pub fn draw_3d_with<U: Uniform>(&mut self, shader: &Shader3D<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let uniforms = uniform.values()
			.into_iter()
			.map(|(n, v)| (n, v.into_uniform()))
			.collect::<Vec<(&'static str, gl::UniformValue)>>();

		flush(self);
		self.cur_pipeline_3d = gl::Pipeline::clone(&shader.gl_pipeline());
		self.cur_custom_uniform_3d = Some(uniforms);
		f(self)?;
		flush(self);
		self.cur_pipeline_3d = self.default_pipeline_3d.clone();
		self.cur_custom_uniform_3d = None;

		return Ok(());

	}

	pub fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

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

	pub fn use_blend(&mut self, b: Blend, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let default = Blend::Alpha.to_gl();
		let b = b.to_gl();

		flush(self);
		self.gl.blend_func(b.0, b.1);
		f(self)?;
		flush(self);
		self.gl.blend_func(default.0, default.1);

		return Ok(());

	}

	pub fn coord(&self, coord: Origin) -> Vec2 {

		let w = self.gwidth();
		let h = self.gheight();
		let orig_pt = self.conf.origin.as_pt();
		let coord_pt = coord.as_pt();

		return (coord_pt - orig_pt) / 2.0 * vec2!(w, h);

	}

	pub fn use_cam(&mut self, cam: &impl Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let oview_3d = self.view_3d;
		let oproj_3d = self.proj_3d;

		self.view_3d = cam.lookat();
		self.proj_3d = cam.projection();

		if self.cur_canvas.is_some() {
			self.proj_3d = self.proj_3d.flip_y();
		}

		f(self)?;

		self.view_3d = oview_3d;
		self.proj_3d = oproj_3d;

		return Ok(());

	}

	pub fn transform(&self) -> Transform {
		return self.transform;
	}

	pub fn to_sc(&self, pt: Vec3) -> Vec2 {

		let pt = self.proj_3d * self.view_3d * self.transform * pt;
		let (gw, gh) = (self.gwidth(), self.gheight());
		let pt = (pt / pt.z * 0.5).xy() * vec2!(gw, -gh) - self.conf.origin.as_pt() * vec2!(gw, gh) / 2.0;

		return pt;

	}

	pub fn default_font(&self) -> &impl Font {
		return &self.default_font;
	}

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
pub struct PerspProj {
	pub fov: f32,
	pub aspect: f32,
	pub near: f32,
	pub far: f32,
}

impl PerspProj {

	pub fn as_mat4(&self) -> Mat4 {

		let f = 1.0 / (self.fov / 2.0).tan();

		return mat4!(
			-f / self.aspect, 0.0, 0.0, 0.0,
			0.0, f, 0.0, 0.0,
			0.0, 0.0, (self.far + self.near) / (self.far - self.near), 1.0,
			0.0, 0.0, -(2.0 * self.far * self.near) / (self.far - self.near), 0.0,
		);

	}

}

#[derive(Debug, Clone, Copy)]
pub struct OrthoProj {
	pub width: f32,
	pub height: f32,
	pub near: f32,
	pub far: f32,
	pub origin: Origin,
}

impl OrthoProj {

	pub fn as_mat4(&self) -> Mat4 {

		use Origin::*;

		let w = self.width as f32;
		let h = self.height as f32;
		let near = self.near;
		let far = self.far;

		let (left, right, bottom, top) = match self.origin {
			TopLeft => (0.0, w, h, 0.0),
			Top => (-w / 2.0, w / 2.0, h, 0.0),
			TopRight => (-w, 0.0, h, 0.0),
			Left => (0.0, w, h / 2.0, -h / 2.0),
			Center => (-w / 2.0, w / 2.0, h / 2.0, -h / 2.0),
			Right => (-w, 0.0, h / 2.0, -h / 2.0),
			BottomLeft => (0.0, w, 0.0, -h),
			Bottom => (-w / 2.0, w / 2.0, 0.0, -h),
			BottomRight => (-w, 0.0, 0.0, -h),
		};

		let tx = -(right + left) / (right - left);
		let ty = -(top + bottom) / (top - bottom);
		let tz = -(far + near) / (far - near);

		return mat4!(
			2.0 / (right - left), 0.0, 0.0, 0.0,
			0.0, 2.0 / (top - bottom), 0.0, 0.0,
			0.0, 0.0, 2.0 / (near - far), 0.0,
			tx, ty, tz, 1.0,
		);

	}

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Blend {
	Alpha,
	Add,
	Replace,
}

impl Blend {
	fn to_gl(&self) -> (gl::BlendFac, gl::BlendFac) {
		return match self {
			Blend::Alpha => (gl::BlendFac::SrcAlpha, gl::BlendFac::OneMinusSrcAlpha),
			Blend::Add => (gl::BlendFac::SrcAlpha, gl::BlendFac::One),
			Blend::Replace => (gl::BlendFac::One, gl::BlendFac::Zero),
		};
	}
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

