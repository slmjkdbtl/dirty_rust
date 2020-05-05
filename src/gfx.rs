// wengwengweng

//! Graphics

use crate::*;
use math::*;

pub use gl::UniformValues;
pub use gl::IntoUniformValue;
pub use gl::FilterMode;
pub use gl::Surface;
pub use gl::Cmp;
pub use gl::Primitive;

pub use texture::*;
pub use transform::*;
pub use shader::*;
pub use canvas::*;
pub use font::*;
pub use camera::*;
pub use model::*;
pub use desc::*;
pub use skybox::*;

pub(crate) const DRAW_COUNT: usize = 65536;
pub(crate) const DEFAULT_NEAR: f32 = -4096.0;
pub(crate) const DEFAULT_FAR: f32 = 4096.0;

pub trait GfxCtx {
	fn device(&self) -> &gl::Device;
}

impl GfxCtx for Ctx {
	fn device(&self) -> &gl::Device {
		return &self.gl;
	}
}

impl GfxCtx for gl::Device {
	fn device(&self) -> &gl::Device {
		return self;
	}
}

pub trait Action = FnOnce(&mut Ctx) -> Result<()>;

impl Ctx {

	pub fn clear(&mut self) {

		self.flush();
		self.gl.clear(Surface::Color);
		self.gl.clear(Surface::Depth);
		self.gl.clear(Surface::Stencil);

	}

	pub fn clear_ex(&mut self, s: Surface) {

		self.flush();
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

	// TODO: viewport 2x scaled with hidpi(false)
	pub fn draw_on(&mut self, canvas: &Canvas, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		if self.cur_canvas.is_some() {
			return Err(format!("cannot use canvas inside a canvas"));
		}

		self.flush();

		let t = self.transform;
		let dpi = self.dpi();
		let (cw, ch) = (canvas.width(), canvas.height());

		self.cur_canvas = Some(canvas.clone());
		self.transform = mat4!();

		self.gl.viewport(
			0,
			0,
			(cw as f32 * dpi) as i32,
			(ch as f32 * dpi) as i32,
		);

		canvas.gl_fbuf().with(|| -> Result<()> {
			f(self)?;
			self.flush();
			return Ok(());
		})?;

		self.cur_canvas = None;
		self.transform = t;

		self.gl.viewport(
			0,
			0,
			(self.width as f32 * dpi) as i32,
			(self.height as f32 * dpi) as i32,
		);

		return Ok(());

	}

	pub fn draw_with<U: CustomUniform>(&mut self, shader: &Shader<U>, uniform: &U, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let uniforms = uniform.values()
			.into_iter()
			.map(|(n, v)| (n, v.into_uniform()))
			.collect::<Vec<(&'static str, gl::UniformValue)>>();

		self.flush();
		self.cur_pipeline = gl::Pipeline::clone(&shader.gl_pipeline());
		self.cur_custom_uniform = Some(uniforms);
		f(self)?;
		self.flush();
		self.cur_pipeline = self.default_pipeline.clone();
		self.cur_custom_uniform = None;

		return Ok(());

	}

	pub fn no_depth_test(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		self.flush();
		self.gl.disable(gl::Capability::DepthTest);
		f(self)?;
		self.flush();
		self.gl.enable(gl::Capability::DepthTest);

		return Ok(());

	}

	pub fn no_depth_write(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		self.flush();
		self.gl.depth_mask(false);
		f(self)?;
		self.flush();
		self.gl.depth_mask(true);

		return Ok(());

	}

	pub fn draw_masked(&mut self, mask: impl FnOnce(&mut Self) -> Result<()>, draw: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let gl = self.gl.clone();

		self.flush();
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

		self.flush();

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

		self.flush();
		gl.disable(gl::Capability::StencilTest);

		return Ok(());

	}

	pub fn use_blend(&mut self, b: Blend, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let default = Blend::Alpha.to_gl();
		let b = b.to_gl();

		self.flush();
		self.gl.blend_func(b.0, b.1);
		f(self)?;
		self.flush();
		self.gl.blend_func(default.0, default.1);

		return Ok(());

	}

	pub fn coord(&self, coord: Origin) -> Vec2 {
		return coord.as_pt() / 2.0 * vec2!(self.width, self.height);
	}

	pub fn use_cam(&mut self, cam: &dyn Camera, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {

		let oview = self.view;
		let oproj = self.proj;

		self.apply_cam(cam);

		f(self)?;

		self.view = oview;
		self.proj = oproj;

		return Ok(());

	}

	pub fn use_default_cam(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
		let cam = self.default_cam();
		return self.use_cam(&cam, f);
	}

	pub fn transform(&self) -> Mat4 {
		return self.transform;
	}

	pub fn default_font(&self) -> &impl Font {
		return &self.default_font;
	}

	pub fn clip_to_screen(&self, p: Vec2) -> Vec2 {
		return p * vec2!(self.width(), self.height()) * 0.5;
	}

	pub fn screen_to_clip(&self, p: Vec2) -> Vec2 {
		return p / 0.5 / vec2!(self.width(), self.height());
	}

	pub fn to_clip(&self, pt: Vec3) -> Vec3 {
		let cp = self.proj * self.view * vec4!(pt.x, pt.y, pt.z, 1.0);
		let cp = cp.xyz() / cp.w;
		return cp;
	}

	pub fn to_screen(&self, p: Vec3) -> Vec2 {
		return self.clip_to_screen(self.to_clip(p).xy());
	}

	pub fn flush(&mut self) {
		self.renderer.flush();
	}

	pub(crate) fn begin_frame(&mut self) {

		self.draw_calls_last = self.draw_calls;
		self.draw_calls = 0;
		self.clear();

	}

	pub(crate) fn end_frame(&mut self) {

		self.flush();
		self.transform = mat4!();
		self.draw_calls += self.renderer.draw_count();
		self.renderer.clear();

	}

	pub(crate) fn apply_cam(&mut self, cam: &dyn Camera) {
		self.proj = cam.proj();
		self.view = cam.view();
	}

	pub(crate) fn default_cam(&mut self) -> impl Camera {

		return OrthoCam {
			width: self.width() as f32,
			height: self.height() as f32,
			near: DEFAULT_NEAR,
			far: DEFAULT_FAR,
		};

	}

}

#[derive(Debug, Clone, Copy)]
pub enum Flip {
	None,
	X,
	Y,
	XY,
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
			Blend::Add => (gl::BlendFac::SrcAlpha, gl::BlendFac::DestAlpha),
			Blend::Replace => (gl::BlendFac::SrcAlpha, gl::BlendFac::Zero),
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

