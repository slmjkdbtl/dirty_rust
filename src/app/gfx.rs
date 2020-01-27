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

	pub fn push(&mut self, t: Mat4, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let ot = self.transform;

		self.transform = ot * t;
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	pub fn reset(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let ot = self.transform;

		self.transform = mat4!();
		f(self)?;
		self.transform = ot;

		return Ok(());

	}

	pub fn draw(&mut self, shape: &impl Drawable) -> Result<()> {
		return shape.draw(self);
	}

	pub fn draw_t(&mut self, t: Mat4, shape: &impl Drawable) -> Result<()> {
		return self.push(t, |ctx| {
			return ctx.draw(shape);
		});
	}

	pub fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		if self.cur_canvas.is_some() {
			return Err(format!("cannot use canvas inside a canvas"));
		}

		flush(self);

		let t = self.transform;
		let dpi = self.dpi();

		self.cur_canvas = Some(canvas.clone());

		self.apply_cam(&OrthoCam::new(
			canvas.width() as f32,
			canvas.height() as f32,
			NEAR,
			FAR,
		));

		self.transform = mat4!();

		self.gl.viewport(0, 0, (canvas.width() as f32 * dpi) as i32, (canvas.height() as f32 * dpi) as i32);

		canvas.gl_fbuf().with(|| -> Result<()> {
			f(self)?;
			flush(self);
			return Ok(());
		})?;

		self.cur_canvas = None;
		self.transform = t;

		self.reset_default_cam();
		self.reset_viewport();

		return Ok(());

	}

	pub(super) fn apply_cam(&mut self, cam: &dyn Camera) {
		self.proj = cam.projection();
		self.view = cam.lookat();
	}

	pub(super) fn reset_default_cam(&mut self) {

		self.apply_cam(&OrthoCam::new(
			self.width() as f32,
			self.height() as f32,
			NEAR,
			FAR,
		));

	}

	fn reset_viewport(&self) {

		let dpi = self.dpi();

		self.gl.viewport(
			0,
			0,
			(self.width as f32 * dpi) as i32,
			(self.height as f32 * dpi) as i32,
		);

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
		return coord.as_pt() / 2.0 * vec2!(self.width, self.height);
	}

	pub fn use_cam(&mut self, cam: &impl Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let oview = self.view;
		let oproj = self.proj;

		self.view = cam.lookat();
		self.proj = cam.projection();

		f(self)?;

		self.view = oview;
		self.proj = oproj;

		return Ok(());

	}

	pub fn transform(&self) -> Mat4 {
		return self.transform;
	}

	pub fn default_font(&self) -> &impl Font {
		return &self.default_font;
	}

}

pub(super) fn begin(ctx: &mut Ctx) {

	ctx.draw_calls_last = ctx.draw_calls;
	ctx.draw_calls = 0;
	ctx.clear();
	ctx.reset_viewport();

}

pub(super) fn end(ctx: &mut Ctx) {

	flush(ctx);
	ctx.transform = mat4!();
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
}

impl OrthoProj {

	pub fn as_mat4(&self) -> Mat4 {

		let w = self.width;
		let h = self.height;
		let near = self.near;
		let far = self.far;

		let (left, right, bottom, top) = (-w / 2.0, w / 2.0, -h / 2.0, h / 2.0);
		let tx = -(right + left) / (right - left);
		let ty = -(top + bottom) / (top - bottom);
		let tz = -(far + near) / (far - near);

		return Mat4::new([
			2.0 / (right - left), 0.0, 0.0, 0.0,
			0.0, 2.0 / (top - bottom), 0.0, 0.0,
			0.0, 0.0, -2.0 / (far - near), 0.0,
			tx, ty, tz, 1.0,
		]);

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
			TopLeft => vec2!(-1, 1),
			Top => vec2!(0, 1),
			TopRight => vec2!(1, 1),
			Left => vec2!(-1, 0),
			Center => vec2!(0, 0),
			Right => vec2!(1, 0),
			BottomLeft => vec2!(-1, -1),
			Bottom => vec2!(0, -1),
			BottomRight => vec2!(1, -1),
		};

	}

}

pub trait Drawable {
	fn draw(&self, ctx: &mut Ctx) -> Result<()>;
}

